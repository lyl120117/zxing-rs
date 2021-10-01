from ctypes import cdll

try:
    lib = cdll.LoadLibrary('target/debug/libzxing_rust.so')
except Exception as e:
    lib = cdll.LoadLibrary('target/release/libzxing_rust.so')



def test_rust_add():
    c = lib.add(2, 3)
    print('c: ', c)

test_rust_add()


def test_rust_decode():
    msg = lib.decode("test.png")
    print('msg: ', msg)


test_rust_decode()