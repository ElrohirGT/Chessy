export const CELL_COLOR = Object.freeze({
    WHITE_COLOR: 'white',
    BLACK_COLOR: 'black'
})

class chessBoard {}

class chessCell {
    constructor(color) {
        this.color = color
        this.piece = ''
    }
}
