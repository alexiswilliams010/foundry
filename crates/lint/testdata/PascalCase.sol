// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StructPascalCaseTest {
    struct PascalCase {
        uint256 a;
    }

    struct PascalCAse {
        uint256 a;
    }

    struct _PascalCase { //~NOTE: structs should use PascalCase
        uint256 a;
    }

    struct pascalCase { //~NOTE: structs should use PascalCase
        uint256 a;
    }

    struct pascalcase { //~NOTE: structs should use PascalCase
        uint256 a;
    }

    struct pascal_case { //~NOTE: structs should use PascalCase
        uint256 a;
    }

    struct PASCAL_CASE { //~NOTE: structs should use PascalCase
        uint256 a;
    }

    struct PASCALCASE { //~NOTE: structs should use PascalCase
        uint256 a;
    }
}

contract contractPascalCaseTest {} //~NOTE: contracts should use PascalCase
contract ContractPascalCaseTest {}
contract _ContractPascalCaseTest {} //~NOTE: contracts should use PascalCase
contract contractpascalcasetest {} //~NOTE: contracts should use PascalCase
contract contract_pascal_case_test {} //~NOTE: contracts should use PascalCase
contract CONTRACT_PASCAL_CASE_TEST {} //~NOTE: contracts should use PascalCase
contract CONTRACTPASCALCASETEST {} //~NOTE: contracts should use PascalCase

library libraryPascalCaseTest {} //~NOTE: libraries should use PascalCase
library LibraryPascalCaseTest {}
library _LibraryPascalCaseTest {} //~NOTE: libraries should use PascalCase
library librarypascalcasetest {} //~NOTE: libraries should use PascalCase
library library_pascal_case_test {} //~NOTE: libraries should use PascalCase
library LIBRARY_PASCAL_CASE_TEST {} //~NOTE: libraries should use PascalCase
library LIBRARYPASCALCASETEST {} //~NOTE: libraries should use PascalCase

// interfaces do not have to adhere to pascal case
interface IPascalCaseTest {}

contract EnumPascalCaseTest {
    enum testcase { one } //~NOTE: enums should use PascalCase
    enum TestCase { one }
    enum _TestCase { one } //~NOTE: enums should use PascalCase
    enum testCase { one } //~NOTE: enums should use PascalCase
    enum test_case { one } //~NOTE: enums should use PascalCase
    enum TEST_CASE { one } //~NOTE: enums should use PascalCase
    enum TESTCASE { one } //~NOTE: enums should use PascalCase
}

event eventPascalCaseTest(); //~NOTE: events should use PascalCase
contract EventPascalCaseTest {
    event EventPascalCaseTest();
    event _EventPascalCaseTest(); //~NOTE: events should use PascalCase
    event eventPascalCaseTest(); //~NOTE: events should use PascalCase
    event eventpascalcasetest(); //~NOTE: events should use PascalCase
    event event_pascal_case_test(); //~NOTE: events should use PascalCase
    event EVENT_PASCAL_CASE_TEST(); //~NOTE: events should use PascalCase
    event EVENTPASCALCASETEST(); //~NOTE: events should use PascalCase
}
