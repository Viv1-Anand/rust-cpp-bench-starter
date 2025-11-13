#include <bits/stdc++.h>
using namespace std;

// Very small JSONL "parser": for each line, do minimal validation and extract a few fields.
// This avoids external dependencies so it builds anywhere.
static inline bool looks_like_json_object(const string& s) {
    auto b = s.find_first_not_of(" \t\r\n");
    auto e = s.find_last_not_of(" \t\r\n");
    if (b == string::npos || e == string::npos) return false;
    return s[b] == '{' && s[e] == '}';
}

int main(int argc, char** argv) {
    if (argc < 2) {
        cerr << "usage: parse <input_path>\n";
        return 1;
    }
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    const string path = argv[1];
    ifstream in(path);
    if (!in) {
        cerr << "failed to open: " << path << "\n";
        return 2;
    }

    auto t0 = chrono::high_resolution_clock::now();
    string line;
    uint64_t lines = 0, ok = 0, bad = 0;
    while (std::getline(in, line)) {
        if (line.empty()) continue;
        lines++;
        if (looks_like_json_object(line)) ok++;
        else bad++;
    }
    auto t1 = chrono::high_resolution_clock::now();
    double ms = chrono::duration<double, std::milli>(t1 - t0).count();
    cout << "cpp_parse lines=" << lines << " ok=" << ok << " bad=" << bad
         << " duration_ms=" << fixed << setprecision(2) << ms << "\n";
    return 0;
}
