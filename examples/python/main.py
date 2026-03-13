from mmex_lib import *
from rich.console import Console
from rich.table import Table

def main():
    db_path = "cp_of_personal_finance.mmb"
    console = Console() # Creamos el objeto de Rich para imprimir

    console.print(f"[bold blue]Usando base de datos:[/bold blue] {db_path}")

    try:
        engine = MmexEngine(db_path, None)
        transactions = engine.transactions().get_all()
        transactions.sort(key=lambda t: float(t.to_amount.v1), reverse=True)

        # Creamos la tabla
        table = Table(title="Mis Transacciones", header_style="bold magenta")

        table.add_column("Monto", justify="right", style="cyan")
        table.add_column("Código", justify="center", style="yellow")
        table.add_column("Notas", style="green")

        for t in transactions:
            # Agregamos cada fila a la tabla
            table.add_row(str(t.to_amount.v1), str(t.trans_code), t.notes)

        # Imprimimos la tabla final
        console.print(table)

    except MmexError.Database as e:
        console.print(f"[red]Error al obtener cuentas:[/red] {e}")
    except Exception as e:
        console.print(f"[bold red]Error inesperado:[/bold red] {type(e).__name__}: {e}")

if __name__ == "__main__":
    main()