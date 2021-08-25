# Model report for file:///tmp/top-repos-quality-repos-xn_ai3kh/myprecious-webtoon.git HEAD 1d37a9c9806131561b81ef312c453480956c0754

### Dump

```json
{'created_at': '2021-08-21 16:31:28',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '7454fd32-3d8d-444f-bd3a-89f9ddd6f36c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xn_ai3kh/myprecious-webtoon.git 1d37a9c9806131561b81ef312c453480956c0754

# javascript
15 rules, avg.len. 5.8
## train
PPCR: 0.896076
### report
macro
{'f1-score': 0.6902022071745174,
 'precision': 0.6995426273402006,
 'recall': 0.6833105383250695,
 'support': 13382}
micro
{'f1-score': 0.9369302047526528,
 'precision': 0.9369302047526528,
 'recall': 0.9369302047526528,
 'support': 13382}
weighted
{'f1-score': 0.9283981967994298,
 'precision': 0.9207495783599738,
 'recall': 0.9369302047526528,
 'support': 13382}
### report_full
macro
{'f1-score': 0.6431790000343103,
 'precision': 0.6995426273402006,
 'recall': 0.5994180975111675,
 'support': 14934}
micro
{'f1-score': 0.8855770589066251,
 'precision': 0.9369302047526528,
 'recall': 0.8395607338958082,
 'support': 14934}
weighted
{'f1-score': 0.8742145196800745,
 'precision': 0.9152068984476351,
 'recall': 0.8395607338958082,
 'support': 14934}
## test
PPCR: 0.891478
### report
macro
{'f1-score': 0.6378262682069653,
 'precision': 0.6306060359861401,
 'recall': 0.6614883559640741,
 'support': 2678}
micro
{'f1-score': 0.8790141896938014,
 'precision': 0.8790141896938013,
 'recall': 0.8790141896938013,
 'support': 2678}
weighted
{'f1-score': 0.8688782835292143,
 'precision': 0.8634724505214412,
 'recall': 0.8790141896938013,
 'support': 2678}
### report_full
macro
{'f1-score': 0.5858298866195998,
 'precision': 0.6306060359861401,
 'recall': 0.5528068757784811,
 'support': 3004}
micro
{'f1-score': 0.8285814853924673,
 'precision': 0.8790141896938013,
 'recall': 0.7836218375499334,
 'support': 3004}
weighted
{'f1-score': 0.817091050137509,
 'precision': 0.8563036527424515,
 'recall': 0.7836218375499334,
 'support': 3004}
```

## javascript
### Summary
10 rules, avg.len. 5.6

| | |
|-|-|
|Min support|166|
|Max support|3726|
|Min confidence|0.9278884530067444|
|Max confidence|0.9975369572639465|

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
                     'min_samples_leaf': 116,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.928. Support: 1255.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 769.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.989. Support: 707.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.978. Support: 205.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.950. Support: 414.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.962. Support: 248.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 250.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |
| 9 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 166.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3726.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.6, "max_conf": 0.9975369572639465, "max_support": 3726, "min_conf": 0.9278884530067444, "min_support": 166, "num_rules": 10}}
```
</details>
