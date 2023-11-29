// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract TicTacToe {
    bool[3][3] public x;
    bool[3][3] public o;
    address payable public x_addr;
    bool public x_withdrew;
    address payable public o_addr;
    bool public o_withdrew;
    address public current_player;
    address payable public winner;
    bool public game_won;
    bool public game_tied;
    bool public bet_matched;
    uint public bet;
    uint public move_timer;

    constructor(uint _y, uint _x, address payable _o_addr) payable {
        if (_y > 2 || _x > 2) {
            revert("Must provide a move within the board bounds!");
        }
        bet = msg.value;
        game_won = false;
        game_tied = false;
        bet_matched = false;
        move_timer = block.timestamp + 3600;
        x_addr = payable(msg.sender);
        o_addr = _o_addr;
        x = [[false, false, false],
             [false, false, false],
             [false, false, false]];
        o = [[false, false, false],
             [false, false, false],
             [false, false, false]];
        x[_y][_x] = true;
        current_player = _o_addr;
    }

    function match_bet() public payable {
        if (msg.value < bet) {
            revert("The matched bet value must match the initial bet!");
        }
        bet_matched = true;
    }

    function takeTurn(uint _y, uint _x) external {
        // if address matches current-player, take their turn & evaluate if win scenario achieved
        if (msg.sender != current_player) {
            revert("Game must be played in-turn!");
        }
        if (!bet_matched) {
            revert("Initial bet must be matched!");
        }
        if (game_won) {
            revert("Game is over!");
        }
        if (_y > 2 || _x > 2) {
            revert("Must provide a move within the board bounds!");
        }
        if (x[_y][_x] == true || o[_y][_x] == true) {
            revert("Move must be on a non-taken square!");
        }
        if (msg.sender != x_addr && msg.sender != o_addr) {
            revert("Move must be taken by a registered player!");
        }
        // update board
        if (msg.sender == x_addr) {
            x[_y][_x] = true;
        } else if (msg.sender == o_addr) {
            o[_y][_x] = true;
        }
        // evaluate board for win scenario 1
        if (x[0][0] == true && x[0][1] == true && x[0][2] == true) {
            game_won = true;
            winner = x_addr;
        }
        if (o[0][0] == true && o[0][1] == true && o[0][2] == true) {
            game_won = true;
            winner = o_addr;
        }
        // alternate current-player
        if (current_player == x_addr) {
            current_player = o_addr;
        } else {
            current_player = x_addr;
        }
        // evaluate for tie scenario
        if ((x[0][0] == true || o[0][0] == true) 
            && (x[0][1] == true || o[0][1] == true)
            && (x[0][2] == true || o[0][2] == true)
            && (x[1][0] == true || o[1][0] == true)
            && (x[1][1] == true || o[1][1] == true)
            && (x[1][2] == true || o[1][2] == true)
            && (x[2][0] == true || o[2][0] == true)
            && (x[2][1] == true || o[2][1] == true)
            && (x[2][2] == true || o[2][2] == true)) {
            game_tied = true;
        }
    }

    function withdrawFunds() external {
        // If player takes too long to play, allow other player to withdraw funds
        if (!game_won && block.timestamp > move_timer) {
            if (current_player == x_addr) {
                o_addr.transfer(address(this).balance);
            } else {
                x_addr.transfer(address(this).balance);
            }
        }
        // Transfer funds to winning player
        if (game_won) {
            winner.transfer(address(this).balance);
        }
        // If tie, allow players to withdraw backing
        if (game_tied && !x_withdrew && (msg.sender == x_addr)) {
            x_addr.transfer(address(this).balance / 2);
        }
        if (game_tied && !o_withdrew && (msg.sender == o_addr)) {
            o_addr.transfer(address(this).balance / 2);
        }
    }
}