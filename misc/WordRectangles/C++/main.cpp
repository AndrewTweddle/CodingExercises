#include <iostream>
#include <map>
#include <vector>
#include <fstream>
#include <chrono>

using std::map;
using std::vector;
using std::string;
using std::pair;
using std::cout;
using std::endl;
using Clock = std::chrono::steady_clock;
using std::chrono::time_point;
using std::chrono::duration_cast;
using std::chrono::milliseconds;

class TrieNode
{
  public:
    TrieNode(): mWordCount(0) {};
    inline map<char, TrieNode>::const_iterator getIter() const { return mSubNodes.cbegin(); }
    inline bool isIterEnded(map<char, TrieNode>::const_iterator & iter) const { iter == mSubNodes.cend(); }
    bool tryToAddString(const string & word);
  private:
    bool tryToAddSubString(const string & word, size_t startPos, size_t maxStartPos);
    
    map<char, TrieNode> mSubNodes;
    size_t mWordCount;
};

bool TrieNode::tryToAddString(const string & word)
{
    size_t maxStartPos = word.length() - 1;
    return tryToAddSubString(word, 0, maxStartPos);
}

bool TrieNode::tryToAddSubString(const string & word, size_t startPos, size_t maxStartPos)
{
    char nextChar = word[startPos];
    
    // If it's the first char in this branch of the trie, then it's not a duplicate
    auto subNodeIter = mSubNodes.find(nextChar);
    bool isNotADuplicate = subNodeIter == mSubNodes.end();
    
    // If we're at the last level a dummy trie node will be created, but won't be used
    if (startPos < maxStartPos)
    {
        isNotADuplicate = mSubNodes[nextChar].tryToAddSubString(word, startPos+1, maxStartPos);
    }
    else
    {
        if (isNotADuplicate)
        {
            // Create the new leaf-level sub-node
            mSubNodes[nextChar].mWordCount = 1;
        }
    }
    
    if (isNotADuplicate)
    {
        mWordCount++;
        return true;
    }
    
    // Duplicate word found
    return false;
}

struct GridSolver
{
    GridSolver(size_t initRowCount, size_t initColCount, map<size_t, TrieNode> & trieByLength);
    bool solveFirstColumn();
    bool solveColumn(size_t colId, const vector<const TrieNode *> & rowTriesInPrevCol);
    
    const TrieNode & rootRowTrie;
    const TrieNode & rootColTrie;
    const size_t rowCount;
    const size_t colCount;
    vector<vector<char>> solution;
};

struct ColumnSolver
{
    ColumnSolver(GridSolver & initGridSolver, size_t initColId, const vector<const TrieNode *> & initRowTriesInPrevCol)
            : gridSolver(initGridSolver),
              colId(initColId),
              rowTriesInPrevCol(initRowTriesInPrevCol)
    {};
    
    bool solveCell(vector<const TrieNode *> & rowTriesInCurrCol, size_t rowId, const TrieNode & currColTrie);
    
    GridSolver & gridSolver;
    size_t colId;
    const vector<const TrieNode *> & rowTriesInPrevCol;
};

void populateMapOfWordsByLength(const string &inputFilePath, map<size_t, vector<string>> & wordsByLength);
void populateMapOfTriesByLength(
        const map<size_t, vector<string>> & wordsByLength, map<size_t, TrieNode> & trieByLength);
void solve(map<size_t, TrieNode> & trieByLength);

int main(int argc, char *argv[])
{
    if (argc <= 1 || argv[1] == "--help")
    {
        cout << "wordrect PathToFileWithAWordPerLine\n";
        cout << "The input file path is an ASCII text file with words on each line." << endl;
        return 0;
    }
    
    time_point<Clock> start = Clock::now();
    
    string inputFilePath(argv[1]);
    map<size_t, vector<string> > wordsByLength;
    populateMapOfWordsByLength(inputFilePath,wordsByLength);
    
    map<size_t, TrieNode> trieByLength;
    populateMapOfTriesByLength(wordsByLength, trieByLength);
    
    solve(trieByLength);
    
    time_point<Clock> end = Clock::now();
    milliseconds duration = duration_cast<milliseconds>(end - start);
    cout << "\nTOTAL DURATION: " << duration.count() << " ms\n";
    
    return 0;
}

void populateMapOfWordsByLength(
        const string &inputFilePath,
        map<size_t, vector<string>> & wordsByLength)
{
    cout << "Reading words from input file (grouping by word length)..." << endl << endl;
    time_point<Clock> start = Clock::now();

    std::ifstream in(inputFilePath);
    
    string word;
    while (in >> word)
    {
        wordsByLength[word.length()].push_back(word);
    }
    in.close();
    
    cout << "Word counts by length:" << endl;
    cout << "======================" << endl;
    for (auto lenIter = wordsByLength.cbegin(); lenIter != wordsByLength.cend(); ++lenIter)
    {
        auto lenWordsMap = *lenIter;
        cout << lenWordsMap.first << ": " << lenWordsMap.second.size() << endl;
    }
    cout << endl;
    
    time_point<Clock> end = Clock::now();
    milliseconds duration = duration_cast<milliseconds>(end - start);
    cout << "Duration: " << duration.count() << " ms\n" << endl;
}

void populateMapOfTriesByLength(
        const map<size_t, vector<string>> &wordsByLength,
        map<size_t, TrieNode> & trieByLength)
{
    cout << "Reading words into tries (by word length)..." << endl << endl;
    time_point<Clock> start = Clock::now();
    
    size_t uniqueWordCount = 0;
    for (auto lenToWords: wordsByLength)
    {
        auto & trie = trieByLength[lenToWords.first];
        for (const auto& wordIter: lenToWords.second)
        {
            if (trie.tryToAddString(wordIter))
            {
                uniqueWordCount++;
            }
        }
    }
    
    time_point<Clock> end = Clock::now();
    milliseconds duration = duration_cast<milliseconds>(end - start);
    cout << "Duration: " << duration.count() << " ms\n";
    cout << "Total unique words: " << uniqueWordCount << endl << endl;
}

bool tryToSolveGrid(size_t rowCount, size_t colCount, map<size_t, TrieNode> & trieByLength);

void solve(map<size_t, TrieNode> & trieByLength)
{
    cout << "solving..." << endl;
    
    // Test with a grid size that took around 20 seconds in the Scala solution
    size_t rowCount = 7;
    size_t colCount = 18;
    
    // Try something much harder, to look for bugs
    // rowCount = 14;
    // colCount = 14;
    tryToSolveGrid(rowCount, colCount, trieByLength);
}

bool tryToSolveGrid(size_t rowCount, size_t colCount, map<size_t, TrieNode> & trieByLength)
{
    cout << "    " << rowCount << " x " << colCount << endl;
    time_point<Clock> start = Clock::now();
    
    GridSolver gridSolver(rowCount, colCount, trieByLength);
    bool isSolved = gridSolver.solveFirstColumn();

    if (!isSolved)
    {
        cout << "        0 solutions found\n";
    }
    
    time_point<Clock> end = Clock::now();
    milliseconds duration = duration_cast<milliseconds>(end - start);
    cout << "        Search duration: " << duration.count() << " ms" << endl;
    
    return isSolved;
}

GridSolver::GridSolver(size_t initRowCount, size_t initColCount, map<size_t, TrieNode> & trieByLength)
    : rootRowTrie(trieByLength[initRowCount]),
      rootColTrie(trieByLength[initColCount]),
      rowCount(initRowCount),
      colCount(initColCount)
{
    solution.resize(initRowCount);
    for (auto &row : solution)
    {
        row.resize(initColCount);
    }
}

bool GridSolver::solveColumn(size_t colId, const vector<const TrieNode *> & rowTriesInPrevCol)
{
    if (colId == colCount)
    {
        // A solution was found
        cout << "        solution found:\n";
        for (const auto &row: solution)
        {
            cout << "            ";
            for (const auto nextChar: row)
            {
                cout << nextChar;
            }
            cout << endl;
        }
        return true;
    }
    
    vector<const TrieNode *> rowTriesInCurrCol(rowCount);
    ColumnSolver nextColSolver(*this, colId, rowTriesInPrevCol);
    
    return nextColSolver.solveCell(rowTriesInCurrCol, 0, rootColTrie);
}

bool GridSolver::solveFirstColumn()
{
    vector<const TrieNode *> rootRowTriesForEachRow(rowCount, &rootRowTrie);
    return solveColumn(0, rootRowTriesForEachRow);
}

bool ColumnSolver::solveCell(vector<const TrieNode *> & rowTriesInCurrCol, size_t rowId, const TrieNode & currColTrie)
{
    const TrieNode & currRowTrie = *rowTriesInPrevCol[rowId];
    auto rowCharIter = currRowTrie.getIter();
    auto colCharIter = currColTrie.getIter();
    bool rowCharsNotDone = !currRowTrie.isIterEnded(rowCharIter);
    bool colCharsNotDone = !currColTrie.isIterEnded(colCharIter);
    while (rowCharsNotDone && colCharsNotDone)
    {
        char colChar = colCharIter->first;
        while (rowCharsNotDone && rowCharIter->first < colChar)
        {
            ++rowCharIter;
            rowCharsNotDone = !currRowTrie.isIterEnded(rowCharIter);
        }
        if (rowCharsNotDone)
        {
            char rowChar = rowCharIter->first;
            if (rowChar > colChar)
            {
                while (colCharsNotDone && rowChar > colCharIter->first)
                {
                    ++colCharIter;
                    colCharsNotDone = !currColTrie.isIterEnded(colCharIter);
                }
                if (colCharsNotDone)
                {
                    colChar = colCharIter->first;
                }
            }
            
            if (colCharsNotDone && rowChar == colChar)
            {
                gridSolver.solution[rowId][colId] = rowChar;
                rowTriesInCurrCol[rowId] = &rowCharIter->second;
                if (rowId == gridSolver.rowCount - 1)
                {
                    // This column is complete. Solve the next column.
                    if (gridSolver.solveColumn(colId + 1, rowTriesInCurrCol))
                    {
                        return true;
                    }
                }
                else
                {
                    // Find solutions for next cell (one row down in same column)
                    const TrieNode & newCurrColTrieNode = colCharIter->second;
                    if (solveCell(rowTriesInCurrCol, rowId + 1, newCurrColTrieNode))
                    {
                        return true;
                    }
                }
                rowCharIter++;
                rowCharsNotDone = !currRowTrie.isIterEnded(rowCharIter);
                colCharIter++;
                colCharsNotDone = !currColTrie.isIterEnded(colCharIter);
            }
        }
    }
    return false;
}
