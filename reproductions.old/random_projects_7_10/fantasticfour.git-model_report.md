# Model report for file:///tmp/top-repos-quality-repos-_1nu3kix/fantasticfour.git HEAD 90ccc5247582b0a7a145ffacddf0c289847e2268

### Dump

```json
{'created_at': '2021-08-21 09:23:58',
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
 'size': '14.1 kB',
 'tags': [],
 'uuid': '6823bbda-7947-426e-b483-d73fbbc71194',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_1nu3kix/fantasticfour.git 90ccc5247582b0a7a145ffacddf0c289847e2268

# javascript
59 rules, avg.len. 4.4
## train
PPCR: 0.999868
### report
macro
{'f1-score': 0.7871625109225064,
 'precision': 0.9252546146794405,
 'recall': 0.7400900293663746,
 'support': 7547}
micro
{'f1-score': 0.8742546707300914,
 'precision': 0.8742546707300914,
 'recall': 0.8742546707300914,
 'support': 7547}
weighted
{'f1-score': 0.8558662707340267,
 'precision': 0.8836385546192824,
 'recall': 0.8742546707300914,
 'support': 7547}
### report_full
macro
{'f1-score': 0.7871419048292514,
 'precision': 0.9252546146794405,
 'recall': 0.7400419801853191,
 'support': 7548}
micro
{'f1-score': 0.8741967538920172,
 'precision': 0.8742546707300914,
 'recall': 0.87413884472708,
 'support': 7548}
weighted
{'f1-score': 0.8558187177761842,
 'precision': 0.8836356658953987,
 'recall': 0.87413884472708,
 'support': 7548}
## test
PPCR: 0.996032
### report
macro
{'f1-score': 0.7583282416851487,
 'precision': 0.8805169121658483,
 'recall': 0.751237280815725,
 'support': 3514}
micro
{'f1-score': 0.8944223107569721,
 'precision': 0.8944223107569721,
 'recall': 0.8944223107569721,
 'support': 3514}
weighted
{'f1-score': 0.8771739868993088,
 'precision': 0.9061237785977126,
 'recall': 0.8944223107569721,
 'support': 3514}
### report_full
macro
{'f1-score': 0.7577599112442797,
 'precision': 0.8805169121658483,
 'recall': 0.7499622557781267,
 'support': 3528}
micro
{'f1-score': 0.8926441351888669,
 'precision': 0.8944223107569721,
 'recall': 0.8908730158730159,
 'support': 3528}
weighted
{'f1-score': 0.8756616745894528,
 'precision': 0.9060598602961825,
 'recall': 0.8908730158730159,
 'support': 3528}
```

## javascript
### Summary
37 rules, avg.len. 4.9

| | |
|-|-|
|Min support|161|
|Max support|3408|
|Min confidence|0.9218958616256714|
|Max confidence|0.9996865391731262|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 93,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.length ≥ 1<br>	∧ -5.reserved = ;<br>	∧ +5.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 381.` |
| 2 | `  -1.length ≥ 1<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.976. Support: 188.` |
| 3 | `  -1.length ≥ 1<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1849.` |
| 4 | `  -1.length ≥ 1<br>	∧ -2.diff_col ≥ 6<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 1834.` |
| 5 | `  -1.length ≥ 1<br>	∧ -2.diff_col ≤ 5<br>	∧ -4.diff_col ≥ 6<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 321.` |
| 6 | `  -1.length = 0<br>⇒ y = "<br>Confidence: 0.997. Support: 185.` |
| 7 | `  -2.length ≤ 36<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +3.reserved not in {)}<br>	∧ +4.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 1463.` |
| 8 | `  -1.roles not in {RIGHT}<br>	∧ -2.length ≤ 36<br>	∧ +1.length ≥ 1<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +3.reserved not in {)}<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 2247.` |
| 9 | `  -2.length ≤ 36<br>	∧ +1.length = 0<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +3.reserved not in {)}<br>	∧ +4.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.985. Support: 170.` |
| 10 | `  +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.968. Support: 202.` |
| 11 | `  -1.roles not in {ASSIGNMENT}<br>	∧ -2.length ≤ 36<br>	∧ -4.diff_col ≥ 7<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 3408.` |
| 12 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.roles not in {BINARY}<br>	∧ +5.length ≥ 4<br>⇒ y = ∅<br>Confidence: 0.943. Support: 1624.` |
| 13 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.roles not in {BINARY}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1247.` |
| 14 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.roles in {STRING}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.968. Support: 202.` |
| 15 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved = )<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 231.` |
| 16 | `  -1.roles not in {LITERAL}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≥ 2<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {)}<br>	∧ +5.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 930.` |
| 17 | `  -1.roles not in {ASSIGNMENT}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +4.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 161.` |
| 18 | `  -1.reserved = )<br>	∧ -1.roles not in {ASSIGNMENT}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +3.length ≥ 9<br>	∧ +4.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 19 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {ASSIGNMENT}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_line ≤ 2<br>	∧ +3.length ≥ 9<br>	∧ +4.roles not in {BLOCK}<br>	∧ +5.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 366.` |
| 20 | `  -1.roles not in {ASSIGNMENT}<br>	∧ -2.diff_offset ≥ 8<br>	∧ -3.diff_offset ≥ 4<br>	∧ +3.length ≤ 8<br>	∧ +4.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2499.` |
| 21 | `  -1.roles not in {ASSIGNMENT}<br>	∧ -2.diff_offset ≤ 7<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 8<br>	∧ +4.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 449.` |
| 22 | `  -1.reserved not in {;}<br>	∧ -2.length ≤ 36<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1546.` |
| 23 | `  -1.reserved not in {;}<br>	∧ -2.length ≤ 36<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {(}<br>	∧ +3.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 517.` |
| 24 | `  -1.reserved not in {;}<br>	∧ -1.roles in {STRING}<br>	∧ -2.diff_col ≥ 2<br>	∧ -5.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 171.` |
| 25 | `  -1.reserved not in {;}<br>	∧ -2.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.length = 0<br>⇒ y = "<br>Confidence: 0.973. Support: 167.` |
| 26 | `  -4.diff_line ≥ 1<br>	∧ -4.diff_offset ≤ 9<br>	∧ +1.length ≥ 2<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 438.` |
| 27 | `  -1.roles not in {ASSIGNMENT}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 13<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 28 | `  +1.length ≤ 1<br>	∧ +3.length ≤ 13<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 343.` |
| 29 | `  ^1.roles in {IDENTIFIER} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1524.` |
| 30 | `  +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 199.` |
| 31 | `  +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved = ;<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 350.` |
| 32 | `  -1.roles not in {BINARY}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -4.diff_col ≥ 7<br>	∧ +3.reserved = ;<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ∅<br>Confidence: 0.972. Support: 305.` |
| 33 | `  -1.roles not in {BINARY}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -4.diff_col ≥ 7<br>	∧ +1.roles in {STRING}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = "<br>Confidence: 0.981. Support: 187.` |
| 34 | `  -1.roles not in {BINARY}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -4.diff_col ≥ 7<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.internal_type = AssignmentExpression<br>⇒ y = ∅<br>Confidence: 0.964. Support: 208.` |
| 35 | `  -1.reserved = =<br>	∧ -4.diff_line ≤ 5<br>	∧ +1.length = 0<br>	∧ +2.internal_type not in {CommentBlock}<br>⇒ y = "<br>Confidence: 0.987. Support: 198.` |
| 36 | `  -1.reserved not in {=}<br>	∧ -1.roles not in {RIGHT}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line ≤ 5<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1595.` |
| 37 | `  -1.reserved not in {=}<br>	∧ -1.roles not in {RIGHT}<br>	∧ -3.reserved not in {)}<br>	∧ -4.diff_line ≤ 5<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 1905.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.864864864864865, "max_conf": 0.9996865391731262, "max_support": 3408, "min_conf": 0.9218958616256714, "min_support": 161, "num_rules": 37}}
```
</details>
