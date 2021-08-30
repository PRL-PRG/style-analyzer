# Model report for file:///tmp/top-repos-quality-repos-chbgofv8/formidable.git HEAD 3d429e00a4e343cbce94440c9f2a9fcc1f03a8bd

### Dump

```json
{'created_at': '2021-08-29 12:49:50',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.3 kB',
 'tags': [],
 'uuid': 'cfed6707-47fa-4099-9d19-81ee5475864e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-chbgofv8/formidable.git 3d429e00a4e343cbce94440c9f2a9fcc1f03a8bd

# javascript
20 rules, avg.len. 6.4
## train
PPCR: 0.893224
### report
macro
{'f1-score': 0.7960605404248587,
 'precision': 0.7999255450093338,
 'recall': 0.7927233407142024,
 'support': 20261}
micro
{'f1-score': 0.9631804945461725,
 'precision': 0.9631804945461725,
 'recall': 0.9631804945461725,
 'support': 20261}
weighted
{'f1-score': 0.9627095111524394,
 'precision': 0.9623921450349884,
 'recall': 0.9631804945461725,
 'support': 20261}
### report_full
macro
{'f1-score': 0.6942600734339568,
 'precision': 0.7999255450093338,
 'recall': 0.6446330445926269,
 'support': 22683}
micro
{'f1-score': 0.9088580476900149,
 'precision': 0.9631804945461725,
 'recall': 0.8603359344002116,
 'support': 22683}
weighted
{'f1-score': 0.8842915834496525,
 'precision': 0.9302936927408454,
 'recall': 0.8603359344002116,
 'support': 22683}
## test
PPCR: 0.895041
### report
macro
{'f1-score': 0.7762885378634399,
 'precision': 0.7800139451625515,
 'recall': 0.773113653461777,
 'support': 4963}
micro
{'f1-score': 0.9455974209147693,
 'precision': 0.9455974209147693,
 'recall': 0.9455974209147693,
 'support': 4963}
weighted
{'f1-score': 0.9445374040422041,
 'precision': 0.943834047709474,
 'recall': 0.9455974209147693,
 'support': 4963}
### report_full
macro
{'f1-score': 0.6953982323130986,
 'precision': 0.7800139451625515,
 'recall': 0.647601117909584,
 'support': 5545}
micro
{'f1-score': 0.8932242101256185,
 'precision': 0.9455974209147693,
 'recall': 0.8463480613165013,
 'support': 5545}
weighted
{'f1-score': 0.8720300710139677,
 'precision': 0.9133250362786055,
 'recall': 0.8463480613165013,
 'support': 5545}
```

## javascript
### Summary
14 rules, avg.len. 6.6

| | |
|-|-|
|Min support|108|
|Max support|6874|
|Min confidence|0.9433701634407043|
|Max confidence|0.9994103908538818|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 848.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 366.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 144.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 433.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {LITERAL, VALUE}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 177.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 361.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 723.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 447.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 362.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 287.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 108.` |
| 12 | `  •••start_col ≤ 15<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 174.` |
| 13 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 2637.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, if, {}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 6874.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.642857142857143, "max_conf": 0.9994103908538818, "max_support": 6874, "min_conf": 0.9433701634407043, "min_support": 108, "num_rules": 14}}
```
</details>
