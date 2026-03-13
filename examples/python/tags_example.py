from mmex_lib import MmexEngine, TagError, MmexError, TagId
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
import os
import shutil


def tags_demo():
    console = Console()
    base_db = "cp_of_personal_finance.mmb"
    db_path = "example_tags.mmb"

    if os.path.exists(db_path):
        os.remove(db_path)

    if not os.path.exists(base_db):
        console.print(f"[bold red]Error:[/bold red] Base database {base_db} not found.")
        return

    shutil.copy(base_db, db_path)

    console.print(Panel("[bold green]Tags Module Demo[/bold green]"))

    try:
        # Initialize engine
        engine = MmexEngine(db_path, None)
        tag_mgr = engine.tags()

        # 1. Create Tags
        console.print("\n[bold]1. Creating Tags...[/bold]")
        tags_to_create = ["Work", "Personal", "Health", "Education"]
        for name in tags_to_create:
            tag = tag_mgr.create(name)
            console.print(f"Created tag: [cyan]{tag.name}[/cyan] (ID: {tag.id.v1})")

        # 2. List all tags
        console.print("\n[bold]2. Listing all tags:[/bold]")
        all_tags = tag_mgr.get_all()
        table = Table(show_header=True, header_style="bold magenta")
        table.add_column("ID", style="dim", width=18)
        table.add_column("Name", min_width=20)

        for t in all_tags:
            table.add_row(str(t.id.v1), t.name)
        console.print(table)

        # 3. Update a tag
        tag_to_update = all_tags[-1]
        console.print(f"\n[bold]3. Updating tag ID {tag_to_update.id.v1}...[/bold]")
        tag_mgr.update(tag_to_update.id.v1, "Updated Tag Name")
        updated_tag = tag_mgr.get_by_id(tag_to_update.id.v1)
        console.print(f"Updated tag: [cyan]{updated_tag.name}[/cyan]")

        # 4. Error Handling: Name Required
        console.print("\n[bold red]4. Error Handling: Creating empty tag[/bold red]")
        try:
            tag_mgr.create("   ")
        except TagError.NameRequired:
            console.print(
                "[yellow]Caught expected error: Tag name is required (from TagError.NameRequired)[/yellow]"
            )
        except MmexError as e:
            console.print(f"[red]Mmex Database Error: {e}[/red]")
        except Exception as e:
            console.print(
                f"[red]Caught unexpected error: {type(e).__name__}: {e}[/red]"
            )

        # 5. Error Handling: Not Found
        console.print(
            "\n[bold red]5. Error Handling: Getting non-existent tag[/bold red]"
        )
        non_existent = tag_mgr.get_by_id(999)
        if non_existent is None:
            console.print(
                "[yellow]Correctly returned None for non-existent ID 999[/yellow]"
            )
        else:
            console.print(
                f"[red]Error: Should have returned None, but got {non_existent}[/red]"
            )

        # 6. Delete a tag
        console.print(f"\n[bold]6. Deleting tag '{updated_tag.name}'...[/bold]")
        tag_mgr.delete(updated_tag.id.v1)
        console.print(f"Tag ID {updated_tag.id.v1} deleted.")

        # Verify deletion
        all_tags_after = tag_mgr.get_all()
        console.print(f"Total tags now: {len(all_tags_after)}")

    except MmexError as e:
        console.print(f"[bold red]Mmex Error:[/bold red] {e}")
    except Exception as e:
        console.print(f"[bold red]Unexpected Error:[/bold red] {type(e).__name__}: {e}")
    finally:
        if os.path.exists(db_path):
            os.remove(db_path)


if __name__ == "__main__":
    tags_demo()
