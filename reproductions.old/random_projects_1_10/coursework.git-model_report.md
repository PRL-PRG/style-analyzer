# Model report for file:///tmp/top-repos-quality-repos-ri36l8us/coursework.git HEAD 35eca505bda907082a222fc60e2af98890d145da

### Dump

```json
{'created_at': '2021-08-22 11:29:46',
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
 'size': '20.9 kB',
 'tags': [],
 'uuid': '0d06a31c-b3de-4ba3-bbf8-51e0eb4d4163',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ri36l8us/coursework.git 35eca505bda907082a222fc60e2af98890d145da

# javascript
84 rules, avg.len. 6.9
## train
PPCR: 0.852531
### report
macro
{'f1-score': 0.29206834949413857,
 'precision': 0.329141332752592,
 'recall': 0.27642456086037837,
 'support': 24824}
micro
{'f1-score': 0.9005800837898807,
 'precision': 0.9005800837898807,
 'recall': 0.9005800837898807,
 'support': 24824}
weighted
{'f1-score': 0.8855237753560377,
 'precision': 0.8803120050407788,
 'recall': 0.9005800837898807,
 'support': 24824}
### report_full
macro
{'f1-score': 0.24551521302030774,
 'precision': 0.329141332752592,
 'recall': 0.2148578334914195,
 'support': 29118}
micro
{'f1-score': 0.8288902895702792,
 'precision': 0.9005800837898807,
 'recall': 0.7677725118483413,
 'support': 29118}
weighted
{'f1-score': 0.7831044710937773,
 'precision': 0.8233237371532626,
 'recall': 0.7677725118483413,
 'support': 29118}
## test
PPCR: 0.879880
### report
macro
{'f1-score': 0.14620980217899343,
 'precision': 0.20043336944745396,
 'recall': 0.1503855260761338,
 'support': 293}
micro
{'f1-score': 0.8020477815699659,
 'precision': 0.8020477815699659,
 'recall': 0.8020477815699659,
 'support': 293}
weighted
{'f1-score': 0.7870819039607677,
 'precision': 0.843291832908715,
 'recall': 0.8020477815699659,
 'support': 293}
### report_full
macro
{'f1-score': 0.1393015971450637,
 'precision': 0.20043336944745396,
 'recall': 0.1410841524938888,
 'support': 333}
micro
{'f1-score': 0.7507987220447284,
 'precision': 0.8020477815699659,
 'recall': 0.7057057057057057,
 'support': 333}
weighted
{'f1-score': 0.7200370228026705,
 'precision': 0.8188893118470583,
 'recall': 0.7057057057057057,
 'support': 333}
```

## javascript
### Summary
31 rules, avg.len. 4.0

| | |
|-|-|
|Min support|145|
|Max support|5079|
|Min confidence|0.9212564826011658|
|Max confidence|0.9982331991195679|

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
| 1 | `  +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 5079.` |
| 2 | `  +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1510.` |
| 3 | `  +1.reserved not in {;}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1124.` |
| 4 | `  -1.reserved = ;<br>	∧ -4.roles in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁻<br>Confidence: 0.922. Support: 147.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 580.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 262.` |
| 7 | `  •••start_col ≤ 15<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 21<br>	∧ +1.reserved not in {(, ), ,, :, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 321.` |
| 8 | `  -1.reserved = [<br>	∧ +1.roles in {NUMBER}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 9 | `  -1.reserved not in {[}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.989. Support: 4989.` |
| 10 | `  +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1443.` |
| 11 | `  +1.reserved not in {;}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 1162.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 572.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 283.` |
| 14 | `  -1.reserved = [<br>	∧ +1.internal_type = NumericLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 15 | `  -1.reserved not in {[}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 4975.` |
| 16 | `  +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1473.` |
| 17 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 572.` |
| 18 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 593.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 261.` |
| 20 | `  -1.reserved = [<br>	∧ +1.roles in {NUMBER}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 21 | `  +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.974. Support: 4985.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |
| 23 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 601.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 278.` |
| 25 | `  -1.reserved = [<br>	∧ +1.internal_type = NumericLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 26 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 511.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 601.` |
| 28 | `  -1.reserved = [<br>	∧ +1.internal_type = NumericLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 29 | `  -1.reserved not in {[}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 4962.` |
| 30 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 476.` |
| 31 | `  -1.reserved = [<br>	∧ +1.roles in {NUMBER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.967741935483871, "max_conf": 0.9982331991195679, "max_support": 5079, "min_conf": 0.9212564826011658, "min_support": 145, "num_rules": 31}}
```
</details>
