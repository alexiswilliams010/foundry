contract Errors {
    error ValueNonZero();

    function requireWithString(uint256 x) public {
        require(x != 0, "Value cannot be zero"); //~NOTE: use if...revert pattern or custom errors with require statements instead of strings
    }

    function requireWithError(uint256 x) public {
        require(x != 0, ValueNonZero());
    }
}