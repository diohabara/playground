def outer_local():
    x = "local outer"
    def inner_local():
        x = "local inner"
        print(x)
    inner_local()
    print(x)

def outer_nonlocal():
    x = "nonlocal outer"
    def inner_nonlocal():
        nonlocal x
        x = "nonlocal inner"
        print(x)
    inner_nonlocal()
    print(x)

outer_local()
print()
outer_nonlocal()
