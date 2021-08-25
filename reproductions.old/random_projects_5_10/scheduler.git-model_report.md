# Model report for file:///tmp/top-repos-quality-repos-gnqspq34/scheduler.git HEAD 1f7eb76b892bdd1132e468bc2f8280c66377e62f

### Dump

```json
{'created_at': '2021-08-21 18:32:38',
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
 'size': '18.5 kB',
 'tags': [],
 'uuid': '01d875eb-51fc-46c2-ad51-b9a00fba3926',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gnqspq34/scheduler.git 1f7eb76b892bdd1132e468bc2f8280c66377e62f

# javascript
25 rules, avg.len. 10.5
## train
PPCR: 0.902391
### report
macro
{'f1-score': 0.6448404880117247,
 'precision': 0.6526450427309409,
 'recall': 0.6393757576225229,
 'support': 47251}
micro
{'f1-score': 0.9563607119426044,
 'precision': 0.9563607119426044,
 'recall': 0.9563607119426044,
 'support': 47251}
weighted
{'f1-score': 0.9536350428862522,
 'precision': 0.9523053866884958,
 'recall': 0.9563607119426044,
 'support': 47251}
### report_full
macro
{'f1-score': 0.5638927242063068,
 'precision': 0.6526450427309409,
 'recall': 0.5190872313569229,
 'support': 52362}
micro
{'f1-score': 0.907291217009828,
 'precision': 0.9563607119426044,
 'recall': 0.8630113441045033,
 'support': 52362}
weighted
{'f1-score': 0.894230433869535,
 'precision': 0.9425958766862585,
 'recall': 0.8630113441045033,
 'support': 52362}
## test
PPCR: 0.509248
### report
macro
{'f1-score': 0.23029941322795047,
 'precision': 0.28568589141779366,
 'recall': 0.30970841239721697,
 'support': 1239}
micro
{'f1-score': 0.844229217110573,
 'precision': 0.8442292171105731,
 'recall': 0.8442292171105731,
 'support': 1239}
weighted
{'f1-score': 0.848689468668494,
 'precision': 0.8580724452469801,
 'recall': 0.8442292171105731,
 'support': 1239}
### report_full
macro
{'f1-score': 0.13292710038625702,
 'precision': 0.28568589141779366,
 'recall': 0.14579607703518455,
 'support': 2433}
micro
{'f1-score': 0.5697167755991286,
 'precision': 0.8442292171105731,
 'recall': 0.4299219071105631,
 'support': 2433}
weighted
{'f1-score': 0.5552080826794289,
 'precision': 0.845254708922213,
 'recall': 0.4299219071105631,
 'support': 2433}
```

## javascript
### Summary
17 rules, avg.len. 9.8

| | |
|-|-|
|Min support|110|
|Max support|12319|
|Min confidence|0.9308943152427673|
|Max confidence|0.9996084570884705|

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
| 1 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.983. Support: 5939.` |
| 2 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 2349.` |
| 3 | `  +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 2201.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1156.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1277.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1038.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.950. Support: 734.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.953. Support: 628.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 407.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 241.` |
| 11 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 544.` |
| 12 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {EXPRESSION, MAP}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 110.` |
| 13 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, {}<br>	∧ -1.roles not in {EXPRESSION, MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 995.` |
| 14 | `  •••start_line = 255<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 262.` |
| 15 | `  •••start_line = 255<br>	∧ -1.diff_col ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {, }}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 615.` |
| 16 | `  •••start_line = 255<br>	∧ -1.diff_col ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 420.` |
| 17 | `  •••start_line = 255<br>	∧ -1.diff_col ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {, }}<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 12319.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.823529411764707, "max_conf": 0.9996084570884705, "max_support": 12319, "min_conf": 0.9308943152427673, "min_support": 110, "num_rules": 17}}
```
</details>
