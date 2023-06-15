export default class formField {
  _fieldName
  _currentValue
  _regexPattern = null

  constructor(fieldName = 'text', regexPattern = null) {
    this._fieldName = fieldName
    this._currentValue = ''
    if (regexPattern != null) this._regexPattern = new RegExp(regexPattern)
  }
  get fieldName() {
    return this._fieldName
  }
  get currentValue() {
    return this._currentValue
  }
  set currentValue(value) {
    this._currentValue = value
  }
  isValid(value) {
    if (this._regexPattern == null) return true
    else return this._regexPattern.test(value)
  }
}
