#include <iostream>
#include <string>

std::string rot(std::string, int);

int main(int argc, char const* argv[]) {
  std::string input;
  std::getline(std::cin, input);

  auto output = rot(input, 13);
  std::cout << output << std::endl;
  return 0;
}

std::string rot(std::string input_str, int rot_num) {
  std::string res = "";
  for (auto& ch : input_str) {
    if ('a' <= ch && ch <= 'z') {
      res += ((ch - 'a') + rot_num) % 26 + 'a';
    } else if ('A' <= ch && ch <= 'Z') {
      res += ((ch - 'A') + rot_num) % 26 + 'A';
    } else {
      res += ch;
    }
  }
  return res;
}
