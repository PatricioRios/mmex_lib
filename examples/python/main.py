import os
from mmex_lib import *


def main():
    db_path = "personal_finance.mmb"

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

        # 1. Obtener todos los tags como objetos
        tags = tag_manager.get_all()
        print(f"\nTags encontrados ({len(tags)}):")
        for tag in tags:
            print(f"- ID: {tag.id.v1}, Name: {tag.name}")

        # 2. Crear un nuevo tag
        new_tag_name = "Test Namespaced Tag"
        print(f"\nCreando tag: '{new_tag_name}'...")
        new_tag = tag_manager.create(new_tag_name)
        print(f"Creado: ID={new_tag.id.v1}, Name={new_tag.name}")

        # 3. Actualizar el tag
        print(f"Actualizando tag {new_tag.id.v1}...")
        tag_manager.update(new_tag.id.v1, "Namespaced Tag Updated")

        # 4. Obtener por ID
        updated_tag = tag_manager.get_by_id(new_tag.id.v1)
        if updated_tag:
            print(f"Tag actualizado: Name={updated_tag.name}")

        # 5. Cuentas (JSON por ahora)
        accounts_json = account_manager.get_all_json()
        print("\nCuentas (JSON):")
        print(accounts_json[:100] + "...")

        # 6. Borrar el tag
        print(f"\nBorrando tag {new_tag.id.v1}...")
        tag_manager.delete(new_tag.id.v1)

        # Verificar borrado
        if tag_manager.get_by_id(new_tag.id.v1) is None:
            print("Tag borrado exitosamente.")

    except MmexError.Database as e:
        print(f"Error de base de datos: {e}")
    except Exception as e:
        print(f"Error inesperado: {type(e).__name__}: {e}")


if __name__ == "__main__":
    main()
