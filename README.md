# EndfieldPuzzleSolver
Puzzle solver for solving Originium Circuitry problem in Arknight: Endfield game.

> [!NOTE]
> If you want to download the release version, please wait until GitHub Actions workflow is finished.

## Example Usage

> [!NOTE]
> These example usage inputs are copied from [Myitian/OriginiumCircuitryPuzzleSolver](https://github.com/Myitian/OriginiumCircuitryPuzzleSolver), because I have already solve most of the puzzle by hand, I can't take screenshot for example anymore.

### Example 1

![Game screen of the example-1](Example-1.png)

> ```
> Grid:
>   1 2 3 4 - column requirements
> 1 . . . .
> 2 . . . .
> 3 . . . .
> 4 . . . .
>  \
>   row requirements
>
> . - Empty
> * - Unavailable
> 0 - Occupied
> Set the grid:
> ```

```
.....
.....
..0..
.....
.....

```

> ```
> Set the column requirements:
> ```

```
3 5 5 5 3
```

> ```
> Set the row requirements:
> ```

```
3 5 5 5 3
```

> ```
> Please input pixels for block A:
> ```

```
.0
000
.0

```

> ```
> Please input pixels for block B:
> ```

```
.00
00
0

```

> ```
> Please input pixels for block C:
> ```

```
.0
000
.0

```

> ```
> Please input pixels for block D:
> ```

```
.00
00
0

```

> ```
> Please input pixels for block E:
> ```

```

```

> ```
> Solution 1
> .DDC.
> DDCCC
> DA0CB
> AAABB
> .ABB.
> ----------------
> Solution 2
> .ABB.
> AAABB
> DA0CB
> DDCCC
> .DDC.
> ----------------
> Solution 3
> .BBC.
> BBCCC
> BA0CD
> AAADD
> .ADD.
> ----------------
> Solution 4
> .CBB.
> CCCBB
> DC0AB
> DDAAA
> .DDA.
> ----------------
> Solution 5
> .ADD.
> AAADD
> BA0CD
> BBCCC
> .BBC.
> ----------------
> Solution 6
> .CDD.
> CCCDD
> BC0AD
> BBAAA
> .BBA.
> ----------------
> Solution 7
> .DDA.
> DDAAA
> DC0AB
> CCCBB
> .CBB.
> ----------------
> Solution 8
> .BBA.
> BBAAA
> BC0AD
> CCCDD
> .CDD.
> ----------------
> ```

### Example 2

![Game screen of the example-2](Example-2.png)

> ```
> Grid:
>   1 2 3 4 - column requirements
> 1 . . . .
> 2 . . . .
> 3 . . . .
> 4 . . . .
>  \
>   row requirements
>
> . - Empty
> * - Unavailable
> 0 - Occupied
> Set the grid:
> ```

```
.....
.....
.....
...*.
....*

```

> ```
> Set the column requirements:
> ```

```
3 5 5 5 3
```

> ```
> Set the row requirements:
> ```

```
3 5 5 5 3
```

> ```
> Please input pixels for block A:
> ```

```
000
0

```

> ```
> Please input pixels for block B:
> ```

```
000
.0

```

> ```
> Please input pixels for block C:
> ```

```
000
.0

```

> ```
> Please input pixels for block D:
> ```

```
00
0.

```

> ```
> Please input pixels for block E:
> ```

```

```

> ```
> Solution 1
> AAAC.
> ABCCC
> BBB..
> DD.*.
> .D..*
> ----------------
> Solution 2
> CAAD.
> CCADD
> CBA..
> BB.*.
> .B..*
> ----------------
> Solution 3
> BAAD.
> BBADD
> BCA..
> CC.*.
> .C..*
> ----------------
> Solution 4
> AAAB.
> ACBBB
> CCC..
> DD.*.
> .D..*
> ----------------
> ```

## Contribution
Welcome to contribute to this project. Feel free to open an issue or submit a pull request.