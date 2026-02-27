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
> Add a component:
> ```

```
.0.
000
.0.

```

> ```
> Add a component:
> ```

```
.00
00.
0..

```

> ```
> Add a component:
> ```

```
.0.
000
.0.

```

> ```
> Add a component:
> ```

```
.00
00.
0..

```

> ```
> Add a component:
> ```

```

```