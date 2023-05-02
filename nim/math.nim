func notZero(a: int): bool = a.bool
func isNeg(a: int): bool = (a and -2147483648).bool

func subtract(a,b: int): int =
    result = a
    var c = b
    while (notZero(c)):
        result = result xor c
        c = c and result
        c = c shl 1
func add(a,b: int): int =
    result = a
    var d = b
    while (notZero(d)):
        var c = result
        result = result xor d
        d = d and c
        d = d shl 1

func equal(a,b:int): bool = not notZero(a xor b)
func greater(a,b:int): bool = isNeg(subtract(b,a))
func less(a,b:int): bool = isNeg(subtract(a,b))
func greaterEqual(a,b:int): bool = greater(a,b) or equal(a,b)
func lessEqual(a,b:int): bool = less(a,b) or equal(a,b)

func multiply(a,b:int): int =
  var c = b
  while(notZero(c)):
    result = add(result,a)
    c = subtract(c,1)
func divide(a,b: int): int =
    if(notZero(b)):
        while(lessEqual(multiply(result,b),a)):
            result = add(result, 1)
    result = subtract(result, 1)
func modulo(a,b:int): int = subtract(a, multiply(divide(a,b), b))

doAssert notZero(4)
doAssert isNeg(-12)
doAssert equal(subtract(3,5), -2)
doAssert equal(add(14,8), 22)
doAssert equal(add(100,42), 142)
doAssert greater(10,8)
doAssert less(10,12)
doAssert equal(divide(8,2),4)
doAssert equal(modulo(10,4),2)
