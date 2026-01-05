from pathlib import Path
s = Path('scripts/docker-build-wrapper.sh').read_text()
import re
pattern = re.compile(r"\b(if|then|elif|else|fi|do|done)\b")
stack=[]
lines=s.splitlines()
for i,line in enumerate(lines, start=1):
    for m in pattern.finditer(line):
        tok=m.group(1)
        if tok=='if':
            stack.append(('if',i))
        elif tok=='fi':
            if stack and stack[-1][0]=='if':
                stack.pop()
            else:
                print('Unmatched fi at line',i)
        elif tok=='do':
            stack.append(('do',i))
        elif tok=='done':
            if stack and stack[-1][0]=='do':
                stack.pop()
            else:
                print('Unmatched done at line',i)

print('Stack end:', stack)