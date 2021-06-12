## Calculator
A lightweight rust calculation parser. Only supports basic syntax, eg. no parentheses or functions beyond basic operators.

The current operators that are supported are
 
| Character used | Function |
| `+` | Add to number |
| `-` | Subtract from number |
| `*` | Multiply number |
| `\` | Divide number |
| `^` | Use an exponent |

**Example:** `2^2+5` would be transformed into 9. It is pretty much just normal math syntax.

Just pass a string reference to the `calculate` function, and a 64 bit floating point number will be returned.