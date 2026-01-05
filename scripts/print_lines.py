import sys
p='.github/workflows/compliance-check.yml'
with open(p,'r',encoding='utf-8') as f:
    for i,l in enumerate(f, start=1):
        sys.stdout.write(f"{i:03}: {l.rstrip()}\n")
