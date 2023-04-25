const a = {b: (x) => (y) => {
    if(x+y=='cd')
        return a
    else
        return {e: {f:'error!!!'} }
}, e: {f:'hey guys'} }

console.log( a.b`c`('d').e['f'] )