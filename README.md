# 项目先导阶段 - 组件化操作系统

基于 [LearningOS 2026s 课程仓库](https://github.com/LearningOS/2026s-oscamp-professional-2026s-arceos-arceos-classroom-2026s-arceos-oscamp)，由 [LearningOS](https://github.com/LearningOS) 统一分配学员仓库、运行真实测试并上传 OpenCamp。

**6 项练习 · 总分：600**

## 学员提交流程

1. 加入 [OpenCamp 秋冬季训练营](https://opencamp.cn/os2edu/camp/2026fall)，并绑定自己的 GitHub 账号。
2. 点击[领取作业仓库](https://github.com/LearningOS/2026a-enroll/issues/new?template=arceos.yml)，点击 **Create** 提交申请；等待机器人回复，然后接受仓库邀请。
3. 克隆分配的仓库，在 `main` 完成实验并 push。
4. 在 Actions 查看各项测试、原始日志、分数和上传结果，再核对 OpenCamp 学员成绩页面。

详细步骤见[学员指南](docs/STUDENT_GUIDE.md)。

## 计分规则

沿用往期各项练习的权重，每项全部通过才获得该项分数。每次提交重新计算全部练习，上传本次实际总分；不是按提交次数累加。部分完成也会上传测得的分数。编译或测试未通过、超时的练习记 0 分；环境准备失败、缺少结果或结果不完整时不上传。

| 练习 | 分值 |
| --- | ---: |
| `print_with_color` | 100 |
| `ramfs_rename` | 100 |
| `alt_alloc` | 100 |
| `support_hashmap` | 100 |
| `sys_map` | 100 |
| `simple_hv` | 100 |

Actions 中测试作业变红表示还有未完成练习；单独的 **Save measured score and upload to OpenCamp** 作业显示成绩同步是否成功。日志出现 `OpenCamp accepted the score (result=1).` 才表示接口接受成绩。

成绩明细同时保存在运行附件和学员仓库的 `gh-pages` 分支的成绩文件，无需启用 GitHub Pages。

## 学员指南

- [学员指南](docs/STUDENT_GUIDE.md)
