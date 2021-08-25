# Model report for file:///tmp/top-repos-quality-repos-zhh05yep/hackoctoberfest.git HEAD 5bd052f6bb8df4f1a622368e336231e7b5c414fb

### Dump

```json
{'created_at': '2021-08-21 05:24:53',
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
 'size': '18.1 kB',
 'tags': [],
 'uuid': 'da4c28fc-5e4e-41ca-ac23-e754e9e3898c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zhh05yep/hackoctoberfest.git 5bd052f6bb8df4f1a622368e336231e7b5c414fb

# javascript
25 rules, avg.len. 9.6
## train
PPCR: 0.907467
### report
macro
{'f1-score': 0.7004951762507303,
 'precision': 0.7243116633735877,
 'recall': 0.683059253022494,
 'support': 25253}
micro
{'f1-score': 0.9630934938423158,
 'precision': 0.9630934938423158,
 'recall': 0.9630934938423158,
 'support': 25253}
weighted
{'f1-score': 0.9620393684906757,
 'precision': 0.9631299669856701,
 'recall': 0.9630934938423158,
 'support': 25253}
### report_full
macro
{'f1-score': 0.637998009031626,
 'precision': 0.7243116633735877,
 'recall': 0.5837890289236607,
 'support': 27828}
micro
{'f1-score': 0.9163730901829279,
 'precision': 0.9630934938423158,
 'recall': 0.8739758516601983,
 'support': 27828}
weighted
{'f1-score': 0.8978453830479397,
 'precision': 0.9336662098957981,
 'recall': 0.8739758516601983,
 'support': 27828}
## test
PPCR: 0.895939
### report
macro
{'f1-score': 0.6368754790546729,
 'precision': 0.669121962429585,
 'recall': 0.6247549932276494,
 'support': 1059}
micro
{'f1-score': 0.9093484419263456,
 'precision': 0.9093484419263456,
 'recall': 0.9093484419263456,
 'support': 1059}
weighted
{'f1-score': 0.9095396897550491,
 'precision': 0.9224009300939349,
 'recall': 0.9093484419263456,
 'support': 1059}
### report_full
macro
{'f1-score': 0.5634578865154993,
 'precision': 0.669121962429585,
 'recall': 0.5248505042146101,
 'support': 1182}
micro
{'f1-score': 0.8594377510040161,
 'precision': 0.9093484419263456,
 'recall': 0.8147208121827412,
 'support': 1182}
weighted
{'f1-score': 0.8406349878111119,
 'precision': 0.8978074457701946,
 'recall': 0.8147208121827412,
 'support': 1182}
```

## javascript
### Summary
19 rules, avg.len. 9.3

| | |
|-|-|
|Min support|100|
|Max support|4324|
|Min confidence|0.9318181872367859|
|Max confidence|0.9993036389350891|

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
| 1 | `  +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 4324.` |
| 2 | `  -2.diff_offset ≥ 4<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1776.` |
| 3 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 103.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1171.` |
| 5 | `  -1.label in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 718.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 694.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 676.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 553.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 527.` |
| 10 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.reserved not in {,, ;}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 178.` |
| 11 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 249.` |
| 13 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles not in {CALLEE}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 2377.` |
| 14 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles not in {CALLEE}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 286.` |
| 15 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -4.reserved not in {function}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 100.` |
| 16 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -2.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 531.` |
| 17 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION, KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles not in {CALLEE}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 4045.` |
| 18 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≥ 26<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.951. Support: 152.` |
| 19 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 113.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.263157894736842, "max_conf": 0.9993036389350891, "max_support": 4324, "min_conf": 0.9318181872367859, "min_support": 100, "num_rules": 19}}
```
</details>
