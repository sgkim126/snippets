class ImplicitClassExample(some: SomeClass) {
  def method = { /* ... */ }
}

implicit final def ImplicitClassExample(some: SomeClass): ImplicitClassExample = new ImplicitClassExample(some)

