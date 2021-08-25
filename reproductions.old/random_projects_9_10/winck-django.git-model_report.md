# Model report for file:///tmp/top-repos-quality-repos-5jzy_qny/winck-django.git HEAD f24e993748f43921124fbf013e9735fd46650b7f

### Dump

```json
{'created_at': '2021-08-20 19:53:28',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.0 kB',
 'tags': [],
 'uuid': 'b1ba23d2-4986-47cf-aa17-64e22214385d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5jzy_qny/winck-django.git f24e993748f43921124fbf013e9735fd46650b7f

# javascript
50 rules, avg.len. 9.7
## train
PPCR: 0.843584
### report
macro
{'f1-score': 0.643597821804822,
 'precision': 0.6820637787097547,
 'recall': 0.6235771024116218,
 'support': 57481}
micro
{'f1-score': 0.9519145456759625,
 'precision': 0.9519145456759625,
 'recall': 0.9519145456759625,
 'support': 57481}
weighted
{'f1-score': 0.948775167591065,
 'precision': 0.9474949572613797,
 'recall': 0.9519145456759625,
 'support': 57481}
### report_full
macro
{'f1-score': 0.5468676612150135,
 'precision': 0.6820637787097547,
 'recall': 0.49693778351771706,
 'support': 68139}
micro
{'f1-score': 0.8711510905906703,
 'precision': 0.9519145456759625,
 'recall': 0.8030202967463567,
 'support': 68139}
weighted
{'f1-score': 0.851373815264431,
 'precision': 0.9229007386777384,
 'recall': 0.8030202967463567,
 'support': 68139}
## test
PPCR: 0.876096
### report
macro
{'f1-score': 0.4579229039906596,
 'precision': 0.48315046655891275,
 'recall': 0.47427435046768845,
 'support': 9793}
micro
{'f1-score': 0.9486367813744512,
 'precision': 0.9486367813744512,
 'recall': 0.9486367813744512,
 'support': 9793}
weighted
{'f1-score': 0.9425282912969989,
 'precision': 0.940229639489832,
 'recall': 0.9486367813744512,
 'support': 9793}
### report_full
macro
{'f1-score': 0.41500936790973486,
 'precision': 0.48315046655891275,
 'recall': 0.3946028598686165,
 'support': 11178}
micro
{'f1-score': 0.8859854084211529,
 'precision': 0.9486367813744512,
 'recall': 0.8310967972803721,
 'support': 11178}
weighted
{'f1-score': 0.8671094204985326,
 'precision': 0.9161508054525753,
 'recall': 0.8310967972803721,
 'support': 11178}
```

## javascript
### Summary
26 rules, avg.len. 9.3

| | |
|-|-|
|Min support|113|
|Max support|10073|
|Min confidence|0.9335442781448364|
|Max confidence|0.9967177510261536|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.979. Support: 10073.` |
| 2 | `  •••start_line ≤ 125<br>	∧ -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.967. Support: 1928.` |
| 3 | `  •••start_line ≤ 132<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.977. Support: 1555.` |
| 4 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1004.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ForStatement<br>⇒ y = ␣<br>Confidence: 0.988. Support: 121.` |
| 6 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.975. Support: 338.` |
| 7 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = )<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.950. Support: 191.` |
| 8 | `  •••start_col ≤ 70<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ +1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 639.` |
| 9 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -2.reserved not in {(}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 2411.` |
| 10 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 287.` |
| 11 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 3281.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 2447.` |
| 13 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 679.` |
| 14 | `  •••start_line ≤ 246<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 203.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 872.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 826.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 662.` |
| 18 | `  •••start_line ≥ 240<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.941. Support: 447.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 113.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, if, {}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 442.` |
| 21 | `  •••start_line ≤ 249<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.934. Support: 158.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 432.` |
| 23 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type = CommentLine<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.950. Support: 149.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -2.label in {<space>}<br>	∧ -5.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 154.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 457.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, if, {, }}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {(}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 8466.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.26923076923077, "max_conf": 0.9967177510261536, "max_support": 10073, "min_conf": 0.9335442781448364, "min_support": 113, "num_rules": 26}}
```
</details>
