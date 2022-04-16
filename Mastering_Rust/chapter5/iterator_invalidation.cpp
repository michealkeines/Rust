#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<int> v {1, 14, 23,34};

    for (auto it = v.begin(); it != v.end(); it++) {
        if ((*it) == 14) {
            v.push_back(-1);
            v.push_back(-1);
            v.push_back(-1);v.push_back(-1);v.push_back(-1);v.push_back(-1);v.push_back(-1);v.push_back(-1);
            v.push_back(-1);v.push_back(-1);v.push_back(-1);v.push_back(-1);
            v.push_back(-1);v.push_back(-1);v.push_back(-1);
            v.push_back(-1);v.push_back(-1);v.push_back(-1);
            v.push_back(-1);v.push_back(-1);v.push_back(-1);
        }
    }

    for (auto it = v.begin(); it != v.end(); it++) {
        cout << (*it) << " ";
    }

    return 0;
}