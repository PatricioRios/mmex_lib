import os
from mmex_lib import *


def main():
    db_path = "cp_of_personal_finance.mmb"

    if not os.path.exists(db_path) or os.path.getsize(db_path) == 0:
        print(f"Error: No se encontró '{db_path}' o está vacío.")
        return

    print(f"Usando base de datos: {db_path}")
    try:
        engine = MmexEngine(db_path, None)

        # Acceso vía Managers
        tag_manager = engine.tags()
        account_manager = engine.accounts()
        support_manager = engine.support()

        # Support
        print(f"DB Version: {support_manager.get_db_version()}")

        # --- TAGS ---
        print("\n--- GESTIÓN DE TAGS ---")
        tags = tag_manager.get_all()
        print(f"Tags encontrados ({len(tags)}):")
        for tag in tags:
            print(f"- ID: {tag.id.v1}, Name: {tag.name}")

        # --- ACCOUNTS ---
        print("\n--- GESTIÓN DE CUENTAS ---")
        accounts = account_manager.get_all()
        print(f"Cuentas encontradas ({len(accounts)}):")
        for acc in accounts:
            acc_type_name = type(acc.account_type).__name__
            print(f"- [{acc.id.v1}] {acc.name} ({acc_type_name})")

            # Obtener balance de la cuenta
            balance = account_manager.get_balance(acc.id.v1)
            print(
                f"  Balance: {balance.current_balance.v1} (Inicial: {balance.initial_balance.v1})"
            )
            if acc.name == "MercadoPago":
                print("  Detalles de Mercado Pago:")
                print(f"    - Currency ID: {acc.currency_id.v1}")
                print(f"    - Status: {acc.status}")
                print(f"    - Notes: {acc.notes}")
                print("\n")
                print("\n")
                print(f"    - balance From function of service: {account_manager.get_balance(acc.id.v1)}")
                print("\n")
                print("\n")



        # Ejemplo de creación de cuenta (comentado para no alterar la DB permanentemente)
        # new_acc = Account(
        #     id=AccountId(v1=0),
        #     name="Nueva Cuenta UniFFI",
        #     account_type=AccountType.CHECKING,
        #     account_num=None,
        #     status=AccountStatus.OPEN,
        #     notes="Creada desde Python",
        #     initial_balance=Money(v1="1500.50"),
        #     currency_id=CurrencyId(v1=1),
        #     favorite=False
        # )
        # created = account_manager.create(new_acc)
        # print(f"\nCreada cuenta: {created.name} con ID {created.id.v1}")

    except MmexError.Database as e:
        print(f"Error de base de datos: {e}")
    except Exception as e:
        print(f"Error inesperado: {type(e).__name__}: {e}")


if __name__ == "__main__":
    main()
