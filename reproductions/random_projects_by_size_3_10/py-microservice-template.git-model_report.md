# Model report for file:///tmp/top-repos-quality-repos-dxxuftfx/py-microservice-template.git HEAD 9a899bbaad60e3025b37ea499d86ba334eb8c01e

### Dump

```json
{'created_at': '2021-08-22 01:34:23',
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
 'uuid': 'f78ee8ad-03cf-4fc4-a0b5-59d3d7d0b63a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dxxuftfx/py-microservice-template.git 9a899bbaad60e3025b37ea499d86ba334eb8c01e

# javascript
17 rules, avg.len. 6.7
## train
PPCR: 0.881039
### report
macro
{'f1-score': 0.6899923955435867,
 'precision': 0.695483365071009,
 'recall': 0.6895831142904885,
 'support': 8991}
micro
{'f1-score': 0.9359359359359359,
 'precision': 0.9359359359359359,
 'recall': 0.9359359359359359,
 'support': 8991}
weighted
{'f1-score': 0.9241020666046235,
 'precision': 0.9149180450534078,
 'recall': 0.9359359359359359,
 'support': 8991}
### report_full
macro
{'f1-score': 0.6037763368817228,
 'precision': 0.695483365071009,
 'recall': 0.5539004080933498,
 'support': 10205}
micro
{'f1-score': 0.8767451552406752,
 'precision': 0.9359359359359359,
 'recall': 0.8245957863792258,
 'support': 10205}
weighted
{'f1-score': 0.8569620845089794,
 'precision': 0.9070299510295503,
 'recall': 0.8245957863792258,
 'support': 10205}
## test
PPCR: 0.935078
### report
macro
{'f1-score': 0.6159687392709072,
 'precision': 0.6499352400392538,
 'recall': 0.6063583363452908,
 'support': 1858}
micro
{'f1-score': 0.7610333692142088,
 'precision': 0.7610333692142088,
 'recall': 0.7610333692142088,
 'support': 1858}
weighted
{'f1-score': 0.6931562036878718,
 'precision': 0.6616742276536104,
 'recall': 0.7610333692142088,
 'support': 1858}
### report_full
macro
{'f1-score': 0.5297641752501745,
 'precision': 0.6499352400392538,
 'recall': 0.5013866957923963,
 'support': 1987}
micro
{'f1-score': 0.7355006501950586,
 'precision': 0.7610333692142088,
 'recall': 0.7116255661801711,
 'support': 1987}
weighted
{'f1-score': 0.6681106448673825,
 'precision': 0.6758245412614355,
 'recall': 0.7116255661801711,
 'support': 1987}
```

## javascript
### Summary
11 rules, avg.len. 8.1

| | |
|-|-|
|Min support|93|
|Max support|2547|
|Min confidence|0.9285714030265808|
|Max confidence|0.9988864064216614|

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
                     'min_samples_split': 227,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.964. Support: 211.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 109.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 117.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.944. Support: 99.` |
| 6 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 93.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 2547.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 449.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 378.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, var, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 161.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 110.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.090909090909092, "max_conf": 0.9988864064216614, "max_support": 2547, "min_conf": 0.9285714030265808, "min_support": 93, "num_rules": 11}}
```
</details>
