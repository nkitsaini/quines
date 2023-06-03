NL = '\n'
x = """z = f'x = "{repr(x)}" + NL + x'\nprint(z)"""
z = f'x = "{repr(x)}"' + NL + x
print(z)
