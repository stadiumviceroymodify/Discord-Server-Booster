defmodule Product do
  defstruct [:name, :price]
end

defmodule Store do
  def add_product(products, name, price) do
    products ++ [%Product{name: name, price: price}]
  end

  def total_value(products) do
    Enum.reduce(products, 0.0, fn product, total ->
      total + product.price
    end)
  end

  def print_report(products) do
    IO.puts("Store Inventory")
    IO.puts("================")

    products
    |> Enum.sort_by(& &1.name)
    |> Enum.each(fn product ->
      IO.puts("#{product.name} | $#{Float.round(product.price, 2)}")
    end)

    IO.puts("================")
    IO.puts("Total Value: $#{Float.round(total_value(products), 2)}")
  end
end

products = []

products =
  products
  |> Store.add_product("Keyboard", 79.99)
  |> Store.add_product("Mouse", 34.50)
  |> Store.add_product("Monitor", 249.00)
  |> Store.add_product("Headset", 64.90)

Store.print_report(products)
