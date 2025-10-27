// WASM Closure Spike Test Runner
// Date: October 23, 2025
// Purpose: Test WASM closure implementations in Node.js

const fs = require('fs');
const { execSync } = require('child_process');

// Convert WAT to WASM using wat2wasm
function convertWatToWasm(watFile, wasmFile) {
  try {
    console.log(`Converting ${watFile} to ${wasmFile}...`);
    execSync(`wat2wasm ${watFile} -o ${wasmFile}`);
    return true;
  } catch (error) {
    console.error(`Error converting WAT to WASM: ${error.message}`);
    return false;
  }
}

// Load and instantiate a WASM module
async function loadWasmModule(wasmFile) {
  try {
    const wasmBuffer = fs.readFileSync(wasmFile);
    const module = await WebAssembly.instantiate(wasmBuffer);
    return module.instance.exports;
  } catch (error) {
    console.error(`Error loading WASM module: ${error.message}`);
    return null;
  }
}

// Test the closure record approach
async function testClosureRecord() {
  const watFile = 'validation/wasm/wasm_closure_record.wat';
  const wasmFile = 'validation/wasm/wasm_closure_record.wasm';
  
  console.log('\n===== Testing Closure Record Approach =====');
  
  if (!convertWatToWasm(watFile, wasmFile)) {
    return false;
  }
  
  const exports = await loadWasmModule(wasmFile);
  if (!exports) {
    return false;
  }
  
  try {
    // Run the test function
    const result = exports._start();
    console.log(`Test result: ${result === 1 ? 'PASSED' : 'FAILED'}`);
    
    // Manual test
    console.log('\nRunning manual test...');
    const counter1 = exports.make_counter();
    const counter2 = exports.make_counter();
    
    // Test counter1
    let result1 = exports.call_closure(counter1, 5);
    let result2 = exports.call_closure(counter1, 3);
    
    // Test counter2
    let result3 = exports.call_closure(counter2, 10);
    
    console.log(`counter1(5) = ${result1} (expected: 5)`);
    console.log(`counter1(3) = ${result2} (expected: 8)`);
    console.log(`counter2(10) = ${result3} (expected: 10)`);
    
    // Additional tests
    result1 = exports.call_closure(counter1, 2);
    result3 = exports.call_closure(counter2, 5);
    
    console.log(`counter1(2) = ${result1} (expected: 10)`);
    console.log(`counter2(5) = ${result3} (expected: 15)`);
    
    return result === 1;
  } catch (error) {
    console.error(`Error running closure record test: ${error.message}`);
    return false;
  }
}

// Test the global variable approach
async function testGlobalVariables() {
  const watFile = 'validation/wasm/wasm_global_closure.wat';
  const wasmFile = 'validation/wasm/wasm_global_closure.wasm';
  
  console.log('\n===== Testing Global Variable Approach =====');
  
  if (!convertWatToWasm(watFile, wasmFile)) {
    return false;
  }
  
  const exports = await loadWasmModule(wasmFile);
  if (!exports) {
    return false;
  }
  
  try {
    // Run the test function
    const result = exports._start();
    console.log(`Test result: ${result === 1 ? 'PASSED' : 'FAILED'}`);
    
    // Manual test
    console.log('\nRunning manual test...');
    const counter1 = exports.make_counter();
    const counter2 = exports.make_counter();
    
    // Test counter1
    let result1 = exports.call_counter(counter1, 5);
    let result2 = exports.call_counter(counter1, 3);
    
    // Test counter2
    let result3 = exports.call_counter(counter2, 10);
    
    console.log(`counter1(5) = ${result1} (expected: 5)`);
    console.log(`counter1(3) = ${result2} (expected: 8)`);
    console.log(`counter2(10) = ${result3} (expected: 10)`);
    
    // Additional tests
    result1 = exports.call_counter(counter1, 2);
    result3 = exports.call_counter(counter2, 5);
    
    console.log(`counter1(2) = ${result1} (expected: 10)`);
    console.log(`counter2(5) = ${result3} (expected: 15)`);
    
    return result === 1;
  } catch (error) {
    console.error(`Error running global variable test: ${error.message}`);
    return false;
  }
}

// Compare performance between the two approaches
async function comparePerformance() {
  const watFile1 = 'validation/wasm/wasm_closure_record.wat';
  const wasmFile1 = 'validation/wasm/wasm_closure_record.wasm';
  const watFile2 = 'validation/wasm/wasm_global_closure.wat';
  const wasmFile2 = 'validation/wasm/wasm_global_closure.wasm';
  
  console.log('\n===== Performance Comparison =====');
  
  // Convert WAT files to WASM
  if (!convertWatToWasm(watFile1, wasmFile1) || !convertWatToWasm(watFile2, wasmFile2)) {
    return;
  }
  
  // Load modules
  const exports1 = await loadWasmModule(wasmFile1);
  const exports2 = await loadWasmModule(wasmFile2);
  
  if (!exports1 || !exports2) {
    return;
  }
  
  // Benchmark parameters
  const iterations = 100000;
  const warmupIterations = 1000;
  
  // Warmup
  console.log(`Warming up with ${warmupIterations} iterations...`);
  
  // Warmup closure record approach
  let counter1 = exports1.make_counter();
  for (let i = 0; i < warmupIterations; i++) {
    exports1.call_closure(counter1, 1);
  }
  
  // Warmup global variable approach
  let counter2 = exports2.make_counter();
  for (let i = 0; i < warmupIterations; i++) {
    exports2.call_counter(counter2, 1);
  }
  
  // Benchmark closure record approach
  console.log(`Benchmarking closure record approach (${iterations} iterations)...`);
  counter1 = exports1.make_counter();
  
  const start1 = performance.now();
  for (let i = 0; i < iterations; i++) {
    exports1.call_closure(counter1, 1);
  }
  const end1 = performance.now();
  const time1 = end1 - start1;
  
  // Benchmark global variable approach
  console.log(`Benchmarking global variable approach (${iterations} iterations)...`);
  counter2 = exports2.make_counter();
  
  const start2 = performance.now();
  for (let i = 0; i < iterations; i++) {
    exports2.call_counter(counter2, 1);
  }
  const end2 = performance.now();
  const time2 = end2 - start2;
  
  // Results
  console.log('\nPerformance Results:');
  console.log(`Closure Record: ${time1.toFixed(2)} ms (${(iterations / time1 * 1000).toFixed(2)} ops/sec)`);
  console.log(`Global Variable: ${time2.toFixed(2)} ms (${(iterations / time2 * 1000).toFixed(2)} ops/sec)`);
  console.log(`Ratio: ${(time1 / time2).toFixed(2)}x (${time1 > time2 ? 'Global Variable faster' : 'Closure Record faster'})`);
}

// Run all tests
async function runTests() {
  console.log('====================================');
  console.log('WASM Closure Compilation Spike Tests');
  console.log('====================================\n');
  
  const recordResult = await testClosureRecord();
  const globalResult = await testGlobalVariables();
  
  if (recordResult && globalResult) {
    console.log('\n✅ Both approaches working correctly!');
    await comparePerformance();
  } else {
    console.log('\n❌ Some tests failed.');
  }
  
  console.log('\nTest completed.');
}

// Run tests
runTests().catch(console.error);