"""Architecture fixture checker only. Not a product codec or independent P0 implementation."""
from pathlib import Path
import hashlib,json,re,unicodedata,uuid
P=Path(__file__).resolve().parents[1]
REG=json.loads((P/'SCHEMAS/REGISTRY.json').read_text()); R=REG['contracts']; PR=REG['primitives']
def require(ok,why):
 if not ok:raise ValueError(why)
def head(major,n):
 require(0<=n<2**64,'range')
 if n<24:return bytes([major*32+n])
 width=1 if n<256 else 2 if n<65536 else 4 if n<2**32 else 8
 return bytes([major*32+{1:24,2:25,4:26,8:27}[width]])+n.to_bytes(width,'big')
def cbor(x):
 if type(x)is bool:return b'\xf5' if x else b'\xf4'
 if type(x)is int:return head(0,x) if x>=0 else head(1,-1-x)
 if isinstance(x,bytes):return head(2,len(x))+x
 if isinstance(x,str):
  require(unicodedata.normalize('NFC',x)==x,'NFC');b=x.encode();return head(3,len(b))+b
 if isinstance(x,list):return head(4,len(x))+b''.join(cbor(v) for v in x)
 raise ValueError('forbidden CBOR type')
def decode(b):
 require(len(b)<=REG['max_record_bytes'],'record bound');i=0
 def item(depth=0):
  nonlocal i
  require(depth<=32 and i<len(b),'depth/truncated');v=b[i];i+=1;m,a=divmod(v,32)
  if m==7:
   require(a in (20,21),'forbidden simple');return a==21
  require(m in (0,1,2,3,4) and a<28,'forbidden major/indefinite')
  if a<24:n=a
  else:
   w={24:1,25:2,26:4,27:8}[a];require(i+w<=len(b),'truncated');n=int.from_bytes(b[i:i+w],'big');i+=w
   require(n>={1:24,2:256,4:65536,8:2**32}[w],'nonminimal')
  if m<2:return n if m==0 else -1-n
  if m==4:
   require(n<=65536,'array bound');return [item(depth+1) for _ in range(n)]
  require(n<=len(b)-i,'truncated');v=b[i:i+n];i+=n
  if m==2:return v
  x=v.decode('utf-8');require(unicodedata.normalize('NFC',x)==x,'NFC');return x
 result=item();require(i==len(b),'trailing');return result
BYTE_TYPES={'digest','bytes','bytes32','bytes64','gitpath'}
def raw(t,x):
 if isinstance(t,dict):
  if 'enum'in t:require(x in t['enum'],'enum');return t['enum'].index(x)
  k=next(k for k in ('option','array','set') if k in t);require(type(x)is list,'array')
  require(len(x)<= (1 if k=='option' else t['max_items']),'array length');v=[raw(t[k],a) for a in x]
  if k=='set':require([cbor(a) for a in v]==sorted(set(cbor(a) for a in v)),'set order/duplicate')
  return v
 if t in R:
  d=R[t];keys=['type','wire_revision']+[f['name'] for f in d['fields']]
  require(type(x)is dict and set(x)==set(keys),'missing/unknown field');require(x['type']==t and type(x['wire_revision'])is int and x['wire_revision']==1,'type/revision')
  return [t,1]+[raw(f['type'],x[f['name']]) for f in d['fields']]
 d=PR[t]
 if 'enum'in d:require(x in d['enum'],'enum');return d['enum'].index(x)
 if t=='bool':require(type(x)is bool,'bool');return x
 if t in ('u16','u32','i16'):require(type(x)is int and d['minimum']<=x<=d['maximum'],'integer width');return x
 require(type(x)is str,'projection string');require(unicodedata.normalize('NFC',x)==x,'NFC')
 if 'pattern'in d:require(re.fullmatch(d['pattern'],x) is not None,'lexical '+t)
 require(len(x)>=d.get('minLength',0) and len(x)<=d.get('maxLength',16777216),'string length')
 if t in ('u64','i64'):v=int(x);require(int(d['x-minimum'])<=v<=int(d['x-maximum']),'integer width');return v
 if t=='uuid':return uuid.UUID(x).bytes
 if t in BYTE_TYPES:return bytes.fromhex(x)
 return x
def projection(t,v):
 if isinstance(t,dict):
  if 'enum'in t:require(type(v)is int and 0<=v<len(t['enum']),'enum');return t['enum'][v]
  k=next(k for k in ('option','array','set') if k in t);require(type(v)is list,'array');return [projection(t[k],a) for a in v]
 if t in R:
  require(type(v)is list and len(v)==len(R[t]['fields'])+2,'record length');require(v[:2]==[t,1],'type/revision')
  return {'type':t,'wire_revision':1,**{f['name']:projection(f['type'],a) for f,a in zip(R[t]['fields'],v[2:])}}
 if 'enum'in PR[t]:require(type(v)is int and 0<=v<len(PR[t]['enum']),'enum');return PR[t]['enum'][v]
 if t=='uuid':require(type(v)is bytes and len(v)==16,'uuid width');return str(uuid.UUID(bytes=v))
 if t in BYTE_TYPES:require(type(v)is bytes,'byte string');return v.hex()
 if t in ('u64','i64'):require(type(v)is int,'integer');return str(v)
 return v
def encode(t,x):return cbor(raw(t,x))
def validate_bytes(t,b):
 x=projection(t,decode(b));require(encode(t,x)==b,'canonical mismatch');return x
def digest(t,b):return hashlib.sha256(('MASTERCODER/V2.1/'+t+'/sha256').encode()+b'\0'+b).hexdigest()
def sample(t,name=''):
 if isinstance(t,dict):
  if 'enum'in t:return t['enum'][0]
  k=next(k for k in ('option','array','set') if k in t)
  return [] if k=='option' else [sample(t[k],name)]
 if t in R:return {'type':t,'wire_revision':1,**{f['name']:sample(f['type'],f['name']) for f in R[t]['fields']}}
 if 'enum'in PR[t]:return PR[t]['enum'][0]
 if t=='uuid':return '11111111-1111-4111-8111-111111111111'
 if t in ('u64','i64'):return '1700000000000000' if name.endswith('_at') or 'deadline'in name else '1'
 if t in ('u16','u32','i16'):return 1
 if t=='bool':return False
 if t in ('digest','bytes32'):return '22'*32
 if t=='bytes64':return '33'*64
 if t=='bytes':return '44'*20 if name=='oid' else '61'
 if t=='gitpath':return b'src/main.rs'.hex()
 if t=='ref':return 'refs/heads/mastercoder/fix/11111111-1111-4111-8111-111111111111'
 if t=='hostname':return 'approve.example.test'
 if t=='origin':return 'https://approve.example.test'
 if t=='currency':return 'USD'
 return 'fixture'
def main():
 vectors=json.loads((P/'TEST_VECTORS/wire-vectors.json').read_text());counts={}
 for v in vectors:
  kind=v['expect'];counts[kind]=counts.get(kind,0)+1
  if kind=='ACCEPT_ENCODING':
   b=encode(v['contract'],v['projection']);require(b.hex()==v['cbor_hex'],'vector bytes '+v['id']);require(digest(v['contract'],b)==v['sha256'],'vector digest');require(validate_bytes(v['contract'],b)==v['projection'],'roundtrip')
  else:
   try:
    if 'projection'in v:encode(v['contract'],v['projection'])
    else:validate_bytes(v['contract'],bytes.fromhex(v['cbor_hex']))
   except (ValueError,TypeError,KeyError,OverflowError,UnicodeError):pass
   else:raise ValueError('unexpected acceptance '+v['id'])
 # Registry references, exact schema projections and declaration coverage.
 def refs(t):
  if isinstance(t,str):require(t in R or t in PR,'undefined '+t)
  elif 'enum'not in t:refs(t[next(k for k in ('option','array','set') if k in t)])
 for name,d in R.items():
  sch=json.loads((P/'SCHEMAS'/f'{name}.schema.json').read_text())
  require(sch['x-cbor-order']==['type','wire_revision']+[f['name'] for f in d['fields']],'schema drift')
  for f in d['fields']:refs(f['type'])
 print(json.dumps({'scope':'LOCAL_ARCHITECTURE_FIXTURE_CHECK_ONLY','contracts':len(R),'vectors':counts,'p0_executed':False},indent=2))
if __name__=='__main__':main()

# Evaluate only declarative synthetic acceptance predicates, not runtime authorities.
def check_semantic_vectors():
 cases=json.loads((P/'TEST_VECTORS/semantic-vectors.json').read_text())
 for v in cases:
  s=v['trusted_test_state'];results=[]
  for rule in v['all_required_predicates']:
   op,a,*rest=rule;x=s[a];y=s[rest[0]] if rest else None
   if op=='eq':ok=x==y
   elif op=='gt':ok=x>y
   elif op=='true':ok=x is True
   elif op=='false':ok=x is False
   elif op=='subset':ok=set(x)<=set(y)
   elif op=='set_eq':ok=set(x)==set(y) and len(x)==len(set(x)) and len(y)==len(set(y))
   elif op=='distinct_at_least':ok=len(set(x))>=y
   elif op=='member':ok=x in y
   else:raise ValueError('unknown predicate '+op)
   results.append(ok)
  require(all(results)==(v['expected']=='ACCEPT_SPEC_PREDICATES'),'semantic fixture '+v['id'])
 print('Semantic specification cases:',len(cases),'(not runtime/P0 evidence)')
if __name__=='__main__':check_semantic_vectors()
