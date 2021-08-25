# Model report for file:///tmp/top-repos-quality-repos-1g02a5z4/poznanaroundyou.git HEAD 80649c30e97e6957d8849118f730aedcc990fae9

### Dump

```json
{'created_at': '2021-08-21 19:11:59',
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
 'size': '17.1 kB',
 'tags': [],
 'uuid': 'c8a15719-6e2c-490e-8f7f-ce8b505d0ebb',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1g02a5z4/poznanaroundyou.git 80649c30e97e6957d8849118f730aedcc990fae9

# javascript
13 rules, avg.len. 6.1
## train
PPCR: 0.873047
### report
macro
{'f1-score': 0.6824902830153673,
 'precision': 0.714849703811921,
 'recall': 0.6600855720800336,
 'support': 13018}
micro
{'f1-score': 0.9437701643877707,
 'precision': 0.9437701643877707,
 'recall': 0.9437701643877707,
 'support': 13018}
weighted
{'f1-score': 0.9361239289451212,
 'precision': 0.9308885289900679,
 'recall': 0.9437701643877707,
 'support': 13018}
### report_full
macro
{'f1-score': 0.5933075261199314,
 'precision': 0.714849703811921,
 'recall': 0.5428148778876312,
 'support': 14911}
micro
{'f1-score': 0.8798023559740772,
 'precision': 0.9437701643877707,
 'recall': 0.8239554691167594,
 'support': 14911}
weighted
{'f1-score': 0.8567956006099641,
 'precision': 0.923652470085599,
 'recall': 0.8239554691167594,
 'support': 14911}
## test
PPCR: 0.851099
### report
macro
{'f1-score': 0.6349543074096121,
 'precision': 0.6453686105091005,
 'recall': 0.6460767485923433,
 'support': 2555}
micro
{'f1-score': 0.8880626223091976,
 'precision': 0.8880626223091976,
 'recall': 0.8880626223091976,
 'support': 2555}
weighted
{'f1-score': 0.8814013913945744,
 'precision': 0.8808381833569792,
 'recall': 0.8880626223091976,
 'support': 2555}
### report_full
macro
{'f1-score': 0.5202980597115145,
 'precision': 0.6453686105091005,
 'recall': 0.4706331097710983,
 'support': 3002}
micro
{'f1-score': 0.8166276768040309,
 'precision': 0.8880626223091976,
 'recall': 0.7558294470353097,
 'support': 3002}
weighted
{'f1-score': 0.7926361822063316,
 'precision': 0.8643861995938783,
 'recall': 0.7558294470353097,
 'support': 3002}
```

## javascript
### Summary
11 rules, avg.len. 6.5

| | |
|-|-|
|Min support|155|
|Max support|3680|
|Min confidence|0.9217267632484436|
|Max confidence|0.9973545074462891|

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
                     'min_samples_leaf': 120,
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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.923. Support: 1258.` |
| 2 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.942. Support: 907.` |
| 3 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 195.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 399.` |
| 5 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.967. Support: 254.` |
| 6 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 186.` |
| 7 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 250.` |
| 8 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 9 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 155.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1054.` |
| 11 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3680.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.454545454545454, "max_conf": 0.9973545074462891, "max_support": 3680, "min_conf": 0.9217267632484436, "min_support": 155, "num_rules": 11}}
```
</details>
