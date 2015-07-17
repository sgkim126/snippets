class DeprecatedName {
  def someMethod(@deprecatedName('oldName) newName: Int) {
  }

  def apply() {
    someMethod(oldName = 3)
    someMethod(newName = 3)
  }
}
