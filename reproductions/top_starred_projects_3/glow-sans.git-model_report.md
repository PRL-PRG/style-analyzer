# Model report for file:///tmp/top-repos-quality-repos-wqaaepdw/glow-sans.git HEAD 52434233ac43f40be3b28730689339d19c8f2964

### Dump

```json
{'created_at': '2021-08-30 02:14:26',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': '043a4110-40da-4fd3-b25c-6766d78f73c2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wqaaepdw/glow-sans.git 52434233ac43f40be3b28730689339d19c8f2964

# javascript
23 rules, avg.len. 7.6
## train
PPCR: 0.901284
### report
macro
{'f1-score': 0.7736080078633876,
 'precision': 0.8071594451560979,
 'recall': 0.7527298610504616,
 'support': 19374}
micro
{'f1-score': 0.9450294208733354,
 'precision': 0.9450294208733354,
 'recall': 0.9450294208733354,
 'support': 19374}
weighted
{'f1-score': 0.9429746285925228,
 'precision': 0.9436825522610977,
 'recall': 0.9450294208733354,
 'support': 19374}
### report_full
macro
{'f1-score': 0.6921811849358304,
 'precision': 0.8071594451560979,
 'recall': 0.6359050920769573,
 'support': 21496}
micro
{'f1-score': 0.8959628089062883,
 'precision': 0.9450294208733354,
 'recall': 0.8517398585783401,
 'support': 21496}
weighted
{'f1-score': 0.8825623336850617,
 'precision': 0.9342752145717051,
 'recall': 0.8517398585783401,
 'support': 21496}
## test
PPCR: 0.881334
### report
macro
{'f1-score': 0.7054412078795218,
 'precision': 0.7272650119058788,
 'recall': 0.698703726436967,
 'support': 3357}
micro
{'f1-score': 0.9055704498063747,
 'precision': 0.9055704498063747,
 'recall': 0.9055704498063747,
 'support': 3357}
weighted
{'f1-score': 0.8997821588623552,
 'precision': 0.9007279352404911,
 'recall': 0.9055704498063747,
 'support': 3357}
### report_full
macro
{'f1-score': 0.6265777350052302,
 'precision': 0.7272650119058788,
 'recall': 0.5829291929115186,
 'support': 3809}
micro
{'f1-score': 0.8484510186994139,
 'precision': 0.9055704498063747,
 'recall': 0.7981097400892623,
 'support': 3809}
weighted
{'f1-score': 0.8305248950271932,
 'precision': 0.889882892571932,
 'recall': 0.7981097400892623,
 'support': 3809}
```

## javascript
### Summary
16 rules, avg.len. 8.3

| | |
|-|-|
|Min support|90|
|Max support|8622|
|Min confidence|0.9236509799957275|
|Max confidence|0.9985422492027283|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ,<br>	∧ -2.reserved not in {)}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 963.` |
| 2 | `  -1.reserved not in {,}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.924. Support: 871.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.986. Support: 331.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.996. Support: 401.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.928. Support: 145.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 460.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type = CallExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 100.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 343.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 153.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 120.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 120.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 105.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -3.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 90.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 257.` |
| 15 | `  •••start_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label in {<space>}<br>	∧ -4.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 176.` |
| 16 | `  •••start_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -4.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 8622.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.3125, "max_conf": 0.9985422492027283, "max_support": 8622, "min_conf": 0.9236509799957275, "min_support": 90, "num_rules": 16}}
```
</details>
