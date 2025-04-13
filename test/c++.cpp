#include <string>
#include <vector>

int main()
{

    // 变量
    int a = 1;
    float b = 2.1;

    // 常量
    const int PI = 3;

    // vector
    std::vector<int> v = { 1, 2, 3 };
    v[1] = 4;

    // string
    const char* cstr = "Hello, World!";
    std::string str = "Hello, World!";

    // new/delete的高亮会被clangd覆盖
    // 需要在clangd的配置文件中添加disabledkind
    int* ptr = new int(5);
    delete ptr;

    return 0;
}
// 函数
int add(int a, int b)
{
    return a + b;
}

// 类
class Person {
public:
    std::string name;
    int age;

    Person(std::string name, int age)
    {
        this->name = name;
        this->age = age;
    }
};

class Human : public Person {
};

// 枚举
enum class Color {
    RED,
    GREEN,
    BLUE
};