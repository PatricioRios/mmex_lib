import os
from mmex_lib import *


def main():
    db_path = "personal_finance.mmb"

    print(f"Usando base de datos: {db_path}")
    try:
        engine = MmexEngine(db_path, None)

        # 1. Obtener todos los tags como objetos
        tags = engine.get_tags()
        print(f"\nTags encontrados ({len(tags)}):")
        for tag in tags:
            # tag.id es un objeto TagId con un campo v1
            print(f"- ID: {tag.id.v1}, Name: {tag.name}")

        # 2. Crear un nuevo tag
        new_tag_name = "Test Tag UniFFI"
        print(f"\nCreando tag: '{new_tag_name}'...")
        new_tag = engine.create_tag(new_tag_name)
        print(f"Creado: ID={new_tag.id.v1}, Name={new_tag.name}")

        # 3. Actualizar el tag
        print(f"Actualizando tag {new_tag.id.v1}...")
        engine.update_tag(new_tag.id.v1, "Test Tag Updated")

        # 4. Obtener por ID
        updated_tag = engine.get_tag_by_id(new_tag.id.v1)
        if updated_tag:
            print(f"Tag actualizado: Name={updated_tag.name}")

        # 5. Borrar el tag
        print(f"Borrando tag {new_tag.id.v1}...")
        engine.delete_tag(new_tag.id.v1)

        # Verificar borrado
        if engine.get_tag_by_id(new_tag.id.v1) is None:
            print("Tag borrado exitosamente.")

    except MmexError.Database as e:
        print(f"Error de base de datos: {e}")
    except Exception as e:
        print(f"Error inesperado: {e}")


if __name__ == "__main__":
    main()
