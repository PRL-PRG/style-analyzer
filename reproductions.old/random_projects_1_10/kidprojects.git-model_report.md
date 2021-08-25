# Model report for file:///tmp/top-repos-quality-repos-hxfwbsbw/kidprojects.git HEAD ecb7937431803842f4097c7782bcf87c69a66f82

### Dump

```json
{'created_at': '2021-08-22 13:42:12',
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
 'size': '20.6 kB',
 'tags': [],
 'uuid': '36dc4855-fab8-4d14-8c1a-720c52e9b123',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-hxfwbsbw/kidprojects.git ecb7937431803842f4097c7782bcf87c69a66f82

# javascript
37 rules, avg.len. 9.0
## train
PPCR: 0.912358
### report
macro
{'f1-score': 0.6067721172952217,
 'precision': 0.6155675540043487,
 'recall': 0.5995257348485737,
 'support': 60555}
micro
{'f1-score': 0.9455536289323755,
 'precision': 0.9455536289323755,
 'recall': 0.9455536289323755,
 'support': 60555}
weighted
{'f1-score': 0.9421785417413113,
 'precision': 0.940653270256435,
 'recall': 0.9455536289323755,
 'support': 60555}
### report_full
macro
{'f1-score': 0.5697524087666856,
 'precision': 0.6155675540043487,
 'recall': 0.5331100483582776,
 'support': 66372}
micro
{'f1-score': 0.9022193859462526,
 'precision': 0.9455536289323755,
 'recall': 0.8626830591213163,
 'support': 66372}
weighted
{'f1-score': 0.8900823732984402,
 'precision': 0.9229482972980626,
 'recall': 0.8626830591213163,
 'support': 66372}
## test
PPCR: 0.832727
### report
macro
{'f1-score': 0.0863975349269467,
 'precision': 0.11535458269329235,
 'recall': 0.1402765814910518,
 'support': 229}
micro
{'f1-score': 0.3668122270742358,
 'precision': 0.36681222707423583,
 'recall': 0.36681222707423583,
 'support': 229}
weighted
{'f1-score': 0.3662707415597218,
 'precision': 0.7429678224297185,
 'recall': 0.36681222707423583,
 'support': 229}
### report_full
macro
{'f1-score': 0.08147627926922942,
 'precision': 0.11535458269329235,
 'recall': 0.13183337087932717,
 'support': 275}
micro
{'f1-score': 0.33333333333333337,
 'precision': 0.36681222707423583,
 'recall': 0.3054545454545455,
 'support': 275}
weighted
{'f1-score': 0.31650092344408,
 'precision': 0.7315481078061723,
 'recall': 0.3054545454545455,
 'support': 275}
```

## javascript
### Summary
25 rules, avg.len. 8.1

| | |
|-|-|
|Min support|99|
|Max support|11803|
|Min confidence|0.9206695556640625|
|Max confidence|0.9998737573623657|

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
                     'min_samples_split': 234,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4585.` |
| 2 | `  -1.reserved = .<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3961.` |
| 3 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.945. Support: 173.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2861.` |
| 5 | `  +1.reserved not in {;}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 2604.` |
| 6 | `  •••start_line = 255<br>	∧ +1.reserved = }<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.985. Support: 99.` |
| 7 | `  -1.reserved = {<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.923. Support: 841.` |
| 8 | `  -1.reserved = {<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.921. Support: 687.` |
| 9 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.954. Support: 1043.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.926. Support: 1038.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 980.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FOR} and not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 126.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1023.` |
| 14 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 948.` |
| 15 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 495.` |
| 16 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 295.` |
| 17 | `  •••start_col ≤ 8<br>	∧ •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 163.` |
| 18 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 887.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {MAP} and not in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 334.` |
| 20 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {MAP, STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 171.` |
| 21 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {MAP, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 671.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {MAP, STRING}<br>	∧ -1.length ≥ 4<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 857.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP, STRING}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 217.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = [<br>	∧ +1.reserved not in {(, ), }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 163.` |
| 25 | `  -1.diff_col ≤ 23<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {, }}<br>	∧ -1.roles not in {MAP, STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 11803.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.08, "max_conf": 0.9998737573623657, "max_support": 11803, "min_conf": 0.9206695556640625, "min_support": 99, "num_rules": 25}}
```
</details>
