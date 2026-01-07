from binascii import unhexlify
h='0aaf010a210a1f0a0c736572766963652e6e616d65120f0a0d74656c656d657472792d6532651289010a0f0a0d74656c656d657472792d65326512760a1013c8'
open('target/test_payload.bin','wb').write(unhexlify(h))
print('wrote target/test_payload.bin')
