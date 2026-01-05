import sys,glob,yaml
ok=True
files=glob.glob('.github/workflows/*.yml')+glob.glob('.github/workflows/*.yaml')
for f in files:
    try:
        with open(f,'r',encoding='utf-8') as fh:
            yaml.safe_load(fh)
        print(f+': OK')
    except Exception as e:
        print(f+': SYNTAX ERROR => '+str(e))
        ok=False
if not ok:
    sys.exit(2)
print('All workflow YAMLs syntactically OK')