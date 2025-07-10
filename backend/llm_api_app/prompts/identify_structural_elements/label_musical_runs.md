#### 🎯 Special Rule Regarding Lyrics:
In musical notation documents, lyrics are often placed on lines directly below notation lines.

If a line consists entirely of spans that would normally be labeled as `"highlight"` (lyrics or words),  
and the **previous line contains any `"notation"` span**,  
you must instead label the entire line as `"notation"`.

This is because such lines visually function as part of the notation.

You are expected to apply this rule consistently.


#### Example:
Input:
```
     
Hymn Title                             Composer Name

     

|: S r R g M P :|  |: d d d m p :|
   Hap-py birth-day to you  

Some commentary below.



More notation below:
|: P- m- g- :|   |: d n s :|
   To the tune of joy-ful song  
```

Output:
```yaml
document:
  - line:
      line-number: 1
      spans: []
  - line:
      line-number: 2
      spans:
        - tag: "highlight"
          start_col: 1
          text: "Hymn Title                             Composer Name"
  - line:
      line-number: 3
      spans: []
  - line:
      line-number: 4
      spans: []
  - line:
      line-number: 5
      spans:
        - tag: "notation"
          start_col: 1
          text: "|: S r R g M P :|  |: d d d m p :|"
  - line:
      line-number: 6
      spans:
        - tag: "notation"
          start_col: 1
          text: "Hap-py birth-day to you"
  - line:
      line-number: 7
      spans:
        - tag: "highlight"
          start_col: 1
          text: "Some commentary below."
  - line:
      line-number: 8
      spans: []
  - line:
      line-number: 9
      spans: []
  - line:
      line-number: 10
      spans: []
  - line:
      line-number: 11
      spans:
        - tag: "highlight"
          start_col: 1
          text: "More notation below:"
  - line:
      line-number: 12
      spans:
        - tag: "notation"
          start_col: 1
          text: "|: P- m- g- :|   |: d n s :|"
  - line:
      line-number: 13
      spans:
        - tag: "notation"
          start_col: 1
          text: "To the tune of joy-ful song"
```

