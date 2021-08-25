# Model report for file:///tmp/top-repos-quality-repos-v0sn10is/diyprojects.git HEAD ec45e529788a55fbb1d69c2790c6ff3b2db4c267

### Dump

```json
{'created_at': '2021-08-22 01:59:58',
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
 'uuid': '2fccdca8-bc9e-4c0d-a62f-127e3e56ef77',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v0sn10is/diyprojects.git ec45e529788a55fbb1d69c2790c6ff3b2db4c267

# javascript
27 rules, avg.len. 6.7
## train
PPCR: 0.954595
### report
macro
{'f1-score': 0.7512817233262525,
 'precision': 0.7950560269469608,
 'recall': 0.7327194490752238,
 'support': 31725}
micro
{'f1-score': 0.9506067769897557,
 'precision': 0.9506067769897557,
 'recall': 0.9506067769897557,
 'support': 31725}
weighted
{'f1-score': 0.9481104410245152,
 'precision': 0.9481113488389729,
 'recall': 0.9506067769897557,
 'support': 31725}
### report_full
macro
{'f1-score': 0.6935975842234369,
 'precision': 0.7950560269469608,
 'recall': 0.6398558431193652,
 'support': 33234}
micro
{'f1-score': 0.9285241459997845,
 'precision': 0.9506067769897557,
 'recall': 0.9074441836673286,
 'support': 33234}
weighted
{'f1-score': 0.9234399554032384,
 'precision': 0.9458605292046318,
 'recall': 0.9074441836673286,
 'support': 33234}
## test
PPCR: 0.955633
### report
macro
{'f1-score': 0.6777028244492239,
 'precision': 0.6982809327591815,
 'recall': 0.6642391311702287,
 'support': 4394}
micro
{'f1-score': 0.9501593081474738,
 'precision': 0.9501593081474738,
 'recall': 0.9501593081474738,
 'support': 4394}
weighted
{'f1-score': 0.9462745695445344,
 'precision': 0.9430544336319999,
 'recall': 0.9501593081474738,
 'support': 4394}
### report_full
macro
{'f1-score': 0.6276232799024968,
 'precision': 0.6982809327591815,
 'recall': 0.5926876573128501,
 'support': 4598}
micro
{'f1-score': 0.9286032028469751,
 'precision': 0.9501593081474738,
 'recall': 0.9080034797738147,
 'support': 4598}
weighted
{'f1-score': 0.9203915151807045,
 'precision': 0.9379412802639456,
 'recall': 0.9080034797738147,
 'support': 4598}
```

## javascript
### Summary
14 rules, avg.len. 6.3

| | |
|-|-|
|Min support|97|
|Max support|12585|
|Min confidence|0.9388984441757202|
|Max confidence|0.998834490776062|

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
| 1 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 189.` |
| 2 | `  -1.reserved = function<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 141.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 97.` |
| 4 | `  •••start_col ≤ 87<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1758.` |
| 5 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 581.` |
| 6 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 506.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 613.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 98.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 403.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 316.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 293.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 130.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, var}<br>	∧ -2.length ≤ 19<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 429.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ -2.length ≤ 19<br>	∧ +1.reserved not in {return, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 12585.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.285714285714286, "max_conf": 0.998834490776062, "max_support": 12585, "min_conf": 0.9388984441757202, "min_support": 97, "num_rules": 14}}
```
</details>
