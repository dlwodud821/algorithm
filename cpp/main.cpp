#include <iostream>
#include <queue>
#include <utility>

int main() {
    char map[5][5] = {
            {'.', '.', '.', '.', '.'},
            {'.', 'S', '#', '.', '.'},
            {'.', '.', '#', '.', '.'},
            {'.', '.', '#', '.', 'G'},
            {'.', '.', '.', '.', '.'}
    };


    bool visited[5][5] =  {
        {false, false, false, false, false},
        {false, false, false, false, false},
        {false, false, false, false, false},
        {false, false, false, false, false},
        {false, false, false, false, false}
    };

    // 현재 칸으로 오기 전에 위치했던 행과 열을 기록하기 위한 배열
    int prevRow[5][5];
    int prevCol[5][5];

    int dist[5][5];

    for (int r = 0; r < 5; r++) {
        for (int c = 0; c < 5; c++) {
            prevRow[r][c] = -1;
            prevCol[r][c] = -1;
            dist[r][c] = -1;
        }
    }

    int dr[4] = {1, -1, 0, 0};
    int dc[4] = {0, 0, 1, -1};

    std::queue<std::pair<int, int>> q;

    int startRow = 1;
    int startCol = 1;
    int goalRow = 3;
    int goalCol = 4;

    dist[startRow][startCol] = 0;

    q.push({startRow, startCol});
    visited[startRow][startCol] = true;

    bool found = false;

    while(!q.empty()) {
        std::pair<int, int> current = q.front();
        q.pop();

        int row = current.first;
        int col = current.second;

        for (int i = 0; i < 4; i++) {
            int nextRow = row + dr[i];
            int nextCol = col + dc[i];

            // 경계 벗어남
            if (nextRow < 0 || nextCol < 0 || nextRow >= 5 || nextCol >= 5) {
                continue;
            }

            // 벽을 만남
            if (map[nextRow][nextCol] == '#') {
                continue;
            }

            if (visited[nextRow][nextCol] == true) {
                continue;
            }

            visited[nextRow][nextCol] = true;
            prevRow[nextRow][nextCol] = row;
            prevCol[nextRow][nextCol] = col;
            dist[nextRow][nextCol] = dist[row][col] + 1;

            if (nextRow == goalRow && nextCol == goalCol) {
                found = true;
                break;
            }

            q.push({nextRow, nextCol});
        }

        if (found) {
            break;
        }
    }

    if (found) {
        int r = goalRow;
        int c = goalCol;

        std::cout << "경로: " << std::endl;

        while(!(r == startRow && c == startCol)) {
            std::cout << "(" << r << ", " << c << ") = " << dist[r][c] << std::endl;

            int tempRow = prevRow[r][c];
            int tempCol = prevCol[r][c];

            r = tempRow;
            c = tempCol;
        }

        std::cout << "(" << startRow << ", " << startCol << ") = " << dist[r][c] << std::endl;
    } else {
        std::cout << "경로를 찾지 못했습니다." << std::endl;
    }

    return 0;
}
