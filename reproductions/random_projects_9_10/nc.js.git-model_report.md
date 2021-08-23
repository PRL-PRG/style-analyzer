# Model report for file:///tmp/top-repos-quality-repos-udpselns/nc.js.git HEAD 8344a77c68bc97791d20b9db902e275ed7473531

### Dump

```json
{'created_at': '2021-08-20 22:09:57',
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
 'size': '19.7 kB',
 'tags': [],
 'uuid': '596ac274-0af1-4ec0-811d-2e1e176d3ea1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-udpselns/nc.js.git 8344a77c68bc97791d20b9db902e275ed7473531

# javascript
75 rules, avg.len. 7.0
## train
PPCR: 0.802944
### report
macro
{'f1-score': 0.34608354172372335,
 'precision': 0.34151879903012966,
 'recall': 0.3511837342463549,
 'support': 30988}
micro
{'f1-score': 0.9278430360139409,
 'precision': 0.9278430360139409,
 'recall': 0.9278430360139409,
 'support': 30988}
weighted
{'f1-score': 0.917442347549151,
 'precision': 0.9075983579161936,
 'recall': 0.9278430360139409,
 'support': 30988}
### report_full
macro
{'f1-score': 0.2826122439800941,
 'precision': 0.34151879903012966,
 'recall': 0.2564229011489844,
 'support': 38593}
micro
{'f1-score': 0.8264325031258533,
 'precision': 0.9278430360139409,
 'recall': 0.745005570958464,
 'support': 38593}
weighted
{'f1-score': 0.7787383709390425,
 'precision': 0.8240063855957914,
 'recall': 0.745005570958464,
 'support': 38593}
## test
PPCR: 0.675306
### report
macro
{'f1-score': 0.31685829917492103,
 'precision': 0.30686091531300025,
 'recall': 0.33071125812410107,
 'support': 5185}
micro
{'f1-score': 0.893731918997107,
 'precision': 0.893731918997107,
 'recall': 0.893731918997107,
 'support': 5185}
weighted
{'f1-score': 0.8901534408228279,
 'precision': 0.8944672030188726,
 'recall': 0.893731918997107,
 'support': 5185}
### report_full
macro
{'f1-score': 0.254121938043802,
 'precision': 0.30686091531300025,
 'recall': 0.2341686186101338,
 'support': 7678}
micro
{'f1-score': 0.7205162092824381,
 'precision': 0.893731918997107,
 'recall': 0.6035425892159416,
 'support': 7678}
weighted
{'f1-score': 0.6522560331578362,
 'precision': 0.7152010410347784,
 'recall': 0.6035425892159416,
 'support': 7678}
```

## javascript
### Summary
26 rules, avg.len. 6.6

| | |
|-|-|
|Min support|136|
|Max support|8211|
|Min confidence|0.9227941036224365|
|Max confidence|0.9985337257385254|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.968. Support: 7774.` |
| 2 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 341.` |
| 3 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 1478.` |
| 4 | `  -1.reserved not in {)}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 1131.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = )<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 136.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 8211.` |
| 7 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 7898.` |
| 8 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 337.` |
| 9 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1423.` |
| 10 | `  -1.reserved not in {), ;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 1120.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 7968.` |
| 12 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 7908.` |
| 13 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 342.` |
| 14 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1533.` |
| 15 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 944.` |
| 16 | `  -1.reserved not in {), ;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 1146.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 7945.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 165.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 7738.` |
| 20 | `  -1.reserved not in {)}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1058.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 8115.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1470.` |
| 23 | `  -1.reserved not in {), ;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1088.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 8089.` |
| 25 | `  -1.reserved not in {)}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 1111.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 7971.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.576923076923077, "max_conf": 0.9985337257385254, "max_support": 8211, "min_conf": 0.9227941036224365, "min_support": 136, "num_rules": 26}}
```
</details>
