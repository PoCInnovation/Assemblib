import pkg from './poc-test-wasm/pkg/poc_test_wasm.js';
const {add, fibonacci} = pkg;



console.log(add(1,4))
console.log(fibonacci(20))