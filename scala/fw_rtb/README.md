# Flywheel RTB(Real Time Bidding)

<https://www.flywheel.jp/blog/learn_online_advertising_through_coding_quiz/>

`Scala`での実装

## 実行

```sbt
sbt run
```

## メモ

- SSP(Supply-Side Platform)
  - ｢出来るだけ高い値段で広告枠を売｣るシステム
- DSP(Demand-Side Platform)
  - ｢できるだけ安い値段で、効果的な広告枠を買｣うシステム
- RTB(Real Time Bidding)
  - SSPとDSP間のオークションの結果，表示される広告を決定するシステム
- フリークエンシーキャップ
  - 同じユーザーに直近に何度も同じ広告を表示しないための回数制限

## 入力

```txt
n m
ad1 bid1 fcap1 limit1
ad2 bid2 fcap2 limit2
…
adn bidn fcapn limitn
user1 price1
user2 price2
…
userm pricem
```
