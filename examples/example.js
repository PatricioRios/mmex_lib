import { dlopen, FFIType, ptr } from "bun:ffi";

// 1. Cargar la librería dinámica (.so en Linux, .dylib en macOS, .dll en Windows)
const path = "./target/debug/libmmex_lib.so"; 

const lib = dlopen(path, {
  mmex_engine_new: {
    args: [FFIType.cstring, FFIType.cstring],
    returns: FFIType.ptr,
  },
  mmex_engine_free: {
    args: [FFIType.ptr],
    returns: FFIType.void,
  },
  mmex_get_accounts_json: {
    args: [FFIType.ptr],
    returns: FFIType.cstring,
  },
  mmex_get_account_balance_json: {
    args: [FFIType.ptr, FFIType.i64],
    returns: FFIType.cstring,
  },
  mmex_free_string: {
    args: [FFIType.ptr],
    returns: FFIType.void,
  },
});

console.log("--- MMEX Lib JS Bridge ---");

// 2. Abrir la base de datos
const dbPath = Buffer.from("personal_finance.mmb\0");
const engine = lib.symbols.mmex_engine_new(dbPath, null);

if (!engine) {
  console.error("Failed to open database");
  process.exit(1);
}

try {
  // 3. Obtener cuentas
  const accountsJson = lib.symbols.mmex_get_accounts_json(engine);
  const accounts = JSON.parse(accountsJson);
  
  console.log(`Found ${accounts.length} accounts.`);
  
  if (accounts.length > 0) {
    const firstAccount = accounts[0];
    console.log(`Account: ${firstAccount.name} (ID: ${firstAccount.id.id})`);
    
    // 4. Obtener balance de la primera cuenta
    const balanceJson = lib.symbols.mmex_get_account_balance_json(engine, BigInt(firstAccount.id.id));
    const balance = JSON.parse(balanceJson);
    
    console.log("Balance Details:");
    console.table({
      Initial: balance.initial_balance[0],
      Deposits: balance.total_deposits[0],
      Withdrawals: balance.total_withdrawals[0],
      Current: balance.current_balance[0]
    });
  }

} catch (e) {
  console.error("Error during execution:", e);
} finally {
  // 5. Limpieza
  lib.symbols.mmex_engine_free(engine);
  console.log("Engine closed.");
}
