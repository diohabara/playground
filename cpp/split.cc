#include <iostream>
#include <string>
#include <vector>

void split(std::vector<std::string>&, std::string&, char);

int main(void) {
  std::vector<std::string> res;
  std::string input;
  char splitter;
  std::cin >> input >> splitter;
  split(res, input, splitter);
  for (auto& str : res) {
    std::cout << str << "\n";
  }
  return 0;
}

void split(std::vector<std::string>& strContainer, std::string& inputStr,
           char splitter = ' ') {
  std::string cur_str = "";
  for (char& ch : inputStr) {
    if (ch == splitter) {
      strContainer.push_back(cur_str);
      cur_str = "";
    } else {
      cur_str += ch;
    }
  }
  if (inputStr.back() != splitter) strContainer.push_back(cur_str);
}
