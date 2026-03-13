from mmex_lib import *
from rich.console import Console
from rich.table import Table

from rich import *

def main():
    db_path = "cp_of_personal_finance.mmb"
    console = Console() # Creamos el objeto de Rich para imprimir

    console.print(f"[bold blue]Usando base de datos:[/bold blue] {db_path}")

    try:
        engine = MmexEngine(db_path, None)

        try:
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

            console.print(table)

            transaction = engine.transactions().get_by_id(None);
            # Imprimimos la tabla final
            console.print("[red]Transacción con ID 0:[/red]", transaction)
            console.print("obteniendo")
        except TransactionError as e:
            console.print(f"[red]Error al obtener transacciones:[/red] {e}")
            match e:
                case TransactionError.Common:
                    console.print("[red]Error común al obtener transacciones.[/red]")
                case TransactionError.NotFound:
                    console.print("[red]No se encontraron transacciones.[/red]")
                case TransactionError.InvalidAmount:
                    console.print("[red]Monto inválido en alguna transacción.[/red]")
                case TransactionError.SplitError:
                    console.print("[red]Error al dividir transacción.[/red]")
            return


    except MmexError.Database as e:
        console.print(f"[red]Error al obtener cuentas:[/red] {e}")
    except Exception as e:
        console.print(f"[bold red]Error inesperado:[/bold red] {type(e).__name__}: {e}")

if __name__ == "__main__":
    main()