from mmex_lib import MmexEngine, CategoryError, MmexError
from rich.console import Console
from rich.table import Table
from rich.tree import Tree
from rich.panel import Panel
import os
import shutil


def categories_demo():
    console = Console()
    base_db = "cp_of_personal_finance.mmb"
    db_path = "example_categories.mmb"

    if os.path.exists(db_path):
        os.remove(db_path)

    if not os.path.exists(base_db):
        console.print(f"[bold red]Error:[/bold red] Base database {base_db} not found.")
        return

    shutil.copy(base_db, db_path)

    console.print(Panel("[bold blue]Categories Module Demo[/bold blue]"))

    try:
        engine = MmexEngine(db_path, None)
        cat_mgr = engine.categories()

        # 1. Create Root Categories
        console.print("\n[bold]1. Creating Root Categories...[/bold]")
        food = cat_mgr.create("Food Example", None)  # None for no parent
        transport = cat_mgr.create("Transport Example", None)
        console.print(
            f"Created: [green]{food.name}[/green], [green]{transport.name}[/green]"
        )

        # 2. Create Subcategories
        console.print("\n[bold]2. Creating Subcategories...[/bold]")
        groceries = cat_mgr.create("Groceries Example", food.id.v1)
        dining = cat_mgr.create("Dining Out Example", food.id.v1)
        console.print(f"Subcategories created for {food.name}")

        # 3. Visualizing with Rich Tree
        console.print("\n[bold]3. Category Structure (Roots only):[/bold]")
        all_cats = cat_mgr.get_all()

        tree = Tree("📁 [bold]Categories[/bold]")
        roots = [c for c in all_cats if c.parent_id is None]
        for root in roots[-5:]:
            branch = tree.add(f"📂 {root.name}")
            subs = cat_mgr.get_subcategories(root.id.v1)
            for sub in subs:
                branch.add(f"📄 {sub.name}")

        console.print(tree)

        # 4. Update Category
        console.print("\n[bold]4. Updating Category...[/bold]")
        groceries.name = "Supermarket Example"
        cat_mgr.update(groceries)
        updated = cat_mgr.get_by_id(groceries.id.v1)
        console.print(f"Renamed to '[cyan]{updated.name}[/cyan]'")

        # 5. Error Handling
        console.print("\n[bold red]5. Error Handling: Invalid Parent ID[/bold red]")
        try:
            # Attempting to create category with invalid parent ID
            # In MMEX, parent_id is often -1 for None, but our FFI uses Optional[int] -> Option<i64>
            # Let's see what happens with a huge ID
            cat_mgr.create("Fail", 999999)
            console.print(
                "[yellow]Category created with non-existent parent (SQLite behavior)[/yellow]"
            )
        except Exception as e:
            console.print(f"[yellow]Caught expected error: {e}[/yellow]")

    except MmexError as e:
        console.print(f"[bold red]Mmex Error:[/bold red] {e}")
    except Exception as e:
        console.print(f"[bold red]Unexpected Error:[/bold red] {e}")
    finally:
        if os.path.exists(db_path):
            os.remove(db_path)


if __name__ == "__main__":
    categories_demo()
