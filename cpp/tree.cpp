#include <iostream>
#include <set>
#include <map>
#include <deque>
#include <queue>
#include <vector>
#include <string>
#include <sstream>
#include <numeric>
#include <algorithm>
#include <functional>
using namespace std;

#define null -1
#define VI(...) std::vector<int>({__VA_ARGS__})
#define VS(...) std::vector<string>({__VA_ARGS__})

template <typename T>
void printVec(const vector<T> &v)
{
    cout << "[";
    for (auto x : v)
        cout << x << ",";
    cout << "]" << endl;
}

template <typename T, short s>
vector<T> &&genVec(const T v[s])
{
    vector<T> vec(v);
    return move(vec);
}

// -----

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

// v: {1, 2, null, 3} => {1, 2, -1, 3}
TreeNode *genTree(const vector<int> &v)
{
    if (v.empty())
        return nullptr;
    deque<TreeNode *> q;
    auto root = new TreeNode(v[0]);
    q.push_back(root);
    for (int i = 1; i < v.size(); ++i)
    {
        TreeNode *l = nullptr, *r = nullptr;
        if (v[i] >= 0)
            l = new TreeNode(v[i]);
        if (i + 1 < v.size())
        {
            i++;
            if (v[i] >= 0)
                r = new TreeNode(v[i]);
        }
        q.front()->left = l;
        q.front()->right = r;
        q.pop_front();
        if (l)
            q.push_back(l);
        if (r)
            q.push_back(r);
    }
    return root;
};

void printTree(const TreeNode *root)
{
    vector<const TreeNode *> vt;
    vt.push_back(root);
    while (!vt.empty())
    {
        vector<const TreeNode *> new_vt;
        int non_null_count = 0;
        for (auto t : vt)
            if (t)
                non_null_count++;
        if (non_null_count == 0)
            break;
        for (auto t : vt)
        {
            if (t && t->val >= 0)
            {
                cout << t->val << ",";
                new_vt.push_back(t->left);
                new_vt.push_back(t->right);
                non_null_count++;
            }
            else
                cout << "null,";
        }
        vt.swap(new_vt); // vt = new_vt
    }
    cout << endl;
}

void printTreePretty(const TreeNode *root)
{
    deque<const TreeNode *> q;
    q.push_back(root);
    while (!q.empty() /* never reaches */)
    {
        int lvlcnt = q.size();
        bool nextlvlnoempty = false;
        cout << "| ";

        for (int i = 0; i < lvlcnt; ++i)
        {
            auto *node = q.front();
            q.pop_front();
            if (!node)
            {
                cout << "- ";
                q.push_back(nullptr);
                q.push_back(nullptr);
            }
            else
            {
                cout << node->val << " ";
                q.push_back(node->left);
                q.push_back(node->right);
                nextlvlnoempty |= node->left || node->right;
            }
        }
        cout << endl;
        if (!nextlvlnoempty)
            break;
    }
}
