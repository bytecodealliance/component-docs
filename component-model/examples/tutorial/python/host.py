from add import Root
from wasmtime import Store

def main():
    store = Store()
    component = Root(store)
    print("1 + 2 =", component.add(store, 1, 2))

if __name__ == '__main__':
    main()
